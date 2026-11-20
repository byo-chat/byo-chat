use byo_domain::message::{AccountId, ConversationId, OutboundId, OutboundText};
use byo_domain::ports::{ConnectorPort, OutboundQueue, Recorded};

#[derive(Debug, PartialEq, Eq)]
pub struct DeliveryReport<C> {
    pub sent: Vec<OutboundId>,
    pub failed: Vec<(OutboundId, C)>,
}

pub fn submit_reply<Q: OutboundQueue>(
    queue: &mut Q,
    message: &OutboundText,
) -> Result<Recorded, Q::Error> {
    queue.enqueue(message)
}

pub fn deliver_pending<Q: OutboundQueue, C: ConnectorPort>(
    queue: &mut Q,
    connector: &mut C,
    account: &AccountId,
) -> Result<DeliveryReport<C::Error>, Q::Error> {
    let mut report = DeliveryReport {
        sent: Vec::new(),
        failed: Vec::new(),
    };
    let mut held: Vec<ConversationId> = Vec::new();
    for message in queue.pending(account)? {
        if held.contains(&message.conversation) {
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
