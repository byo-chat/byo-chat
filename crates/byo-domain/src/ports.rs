use crate::message::{AccountId, InboundText, OutboundId, OutboundText};

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
    fn enqueue(&mut self, message: &OutboundText) -> Result<Recorded, Self::Error>;
    fn pending(&mut self, account: &AccountId) -> Result<Vec<OutboundText>, Self::Error>;
    fn mark_sent(&mut self, id: &OutboundId) -> Result<(), Self::Error>;
}

pub trait ConnectorPort {
    type Error;
    fn send(&mut self, message: &OutboundText) -> Result<(), Self::Error>;
}
