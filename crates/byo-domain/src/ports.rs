use crate::message::InboundText;

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
