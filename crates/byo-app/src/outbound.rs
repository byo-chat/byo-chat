use byo_domain::message::{AccountId, ConversationId, OutboundId, OutboundText, Timestamp};
use byo_domain::ports::{Clock, ConnectorPort, OutboundQueue, Recorded, Rng};

pub const DEFAULT_DEADLINE_SECONDS: i64 = 24 * 60 * 60;
pub const MAX_BACKOFF_SECONDS: i64 = 5 * 60;

pub fn backoff_seconds(attempts: u32, rng: &mut impl Rng) -> i64 {
    let base = 1_i64
        .checked_shl(attempts.saturating_sub(1))
        .map_or(MAX_BACKOFF_SECONDS, |delay| delay.min(MAX_BACKOFF_SECONDS));
    (base + rng.below(base / 2 + 1)).min(MAX_BACKOFF_SECONDS)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeliveryPolicy {
    pub deadline_seconds: i64,
}

impl Default for DeliveryPolicy {
    fn default() -> Self {
        Self {
            deadline_seconds: DEFAULT_DEADLINE_SECONDS,
        }
    }
}

#[derive(Debug)]
pub struct Timing<'a, K, R> {
    pub clock: &'a K,
    pub rng: &'a mut R,
    pub policy: DeliveryPolicy,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DeliveryReport<C> {
    pub sent: Vec<OutboundId>,
    pub failed: Vec<(OutboundId, C)>,
    pub dead_lettered: Vec<OutboundId>,
}

pub fn submit_reply<Q: OutboundQueue>(
    queue: &mut Q,
    clock: &impl Clock,
    message: &OutboundText,
) -> Result<Recorded, Q::Error> {
    queue.enqueue(message, clock.now())
}

pub fn deliver_pending<Q: OutboundQueue, C: ConnectorPort>(
    queue: &mut Q,
    connector: &mut C,
    timing: &mut Timing<'_, impl Clock, impl Rng>,
    account: &AccountId,
) -> Result<DeliveryReport<C::Error>, Q::Error> {
    let Timing { clock, rng, policy } = timing;
    let now = clock.now();
    let mut report = DeliveryReport {
        sent: Vec::new(),
        failed: Vec::new(),
        dead_lettered: Vec::new(),
    };
    let mut held: Vec<ConversationId> = Vec::new();
    for pending in queue.pending(account)? {
        let message = pending.message;
        if held.contains(&message.conversation) {
            continue;
        }
        if now.0 - pending.enqueued_at.0 > policy.deadline_seconds {
            queue.dead_letter(&message.id)?;
            report.dead_lettered.push(message.id);
            continue;
        }
        if pending.next_attempt_at > now {
            held.push(message.conversation);
            continue;
        }
        match connector.send(&message) {
            Ok(()) => {
                queue.mark_sent(&message.id)?;
                report.sent.push(message.id);
            }
            Err(error) => {
                let wait = backoff_seconds(pending.attempts + 1, *rng);
                queue.record_failure(&message.id, Timestamp(now.0 + wait))?;
                held.push(message.conversation);
                report.failed.push((message.id, error));
            }
        }
    }
    Ok(report)
}
