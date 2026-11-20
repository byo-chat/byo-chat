use byo_domain::message::{AccountId, ConversationId, OutboundId, OutboundText};
use byo_domain::ports::{Clock, ConnectorPort, OutboundQueue, Recorded};

pub const DEFAULT_DEADLINE_SECONDS: i64 = 24 * 60 * 60;

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
    clock: &impl Clock,
    account: &AccountId,
    policy: DeliveryPolicy,
) -> Result<DeliveryReport<C::Error>, Q::Error> {
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
        match connector.send(&message) {
            Ok(()) => {
                queue.mark_sent(&message.id)?;
                report.sent.push(message.id);
            }
            Err(error) => {
                held.push(message.conversation);
                report.failed.push((message.id, error));
            }
        }
    }
    Ok(report)
}
