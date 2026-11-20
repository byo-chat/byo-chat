use crate::message::InboundText;

pub trait EventStore {
    type Error;
    fn record_inbound(&mut self, message: &InboundText) -> Result<(), Self::Error>;
}

pub trait DestinationPort {
    type Error;
    fn deliver(&mut self, message: &InboundText) -> Result<(), Self::Error>;
}
