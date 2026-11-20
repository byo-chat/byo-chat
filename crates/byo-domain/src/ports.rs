use crate::message::{
    AccountId, InboundText, OutboundId, OutboundText, PendingOutbound, Timestamp,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recorded {
    New,
    Duplicate,
}

pub trait EventStore {
    type Error;
    fn record_inbound(&mut self, message: &InboundText) -> Result<Recorded, Self::Error>;
}

pub trait DestinationPort {
    type Error;
    fn deliver(&mut self, message: &InboundText) -> Result<(), Self::Error>;
}

pub trait OutboundQueue {
    type Error;
    fn enqueue(&mut self, message: &OutboundText, at: Timestamp) -> Result<Recorded, Self::Error>;
    fn pending(&mut self, account: &AccountId) -> Result<Vec<PendingOutbound>, Self::Error>;
    fn mark_sent(&mut self, id: &OutboundId) -> Result<(), Self::Error>;
    fn dead_letter(&mut self, id: &OutboundId) -> Result<(), Self::Error>;
    fn record_failure(
        &mut self,
        id: &OutboundId,
        next_attempt_at: Timestamp,
    ) -> Result<(), Self::Error>;
}

pub trait ConnectorPort {
    type Error;
    fn send(&mut self, message: &OutboundText) -> Result<(), Self::Error>;
}

pub trait Clock {
    fn now(&self) -> Timestamp;
}

pub trait Rng {
    fn below(&mut self, upper: i64) -> i64;
}
