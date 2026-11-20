use byo_domain::message::{AccountId, OutboundText};
use byo_domain::ports::{ConnectorPort, OutboundQueue, Recorded};

#[derive(Debug, PartialEq, Eq)]
pub enum OutboundError<Q, C> {
    Queue(Q),
    Connector(C),
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
) -> Result<(), OutboundError<Q::Error, C::Error>> {
    for message in queue.pending(account).map_err(OutboundError::Queue)? {
        connector.send(&message).map_err(OutboundError::Connector)?;
        queue.mark_sent(&message.id).map_err(OutboundError::Queue)?;
    }
    Ok(())
}
