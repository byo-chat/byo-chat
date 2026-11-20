use byo_domain::message::InboundText;
use byo_domain::ports::{DestinationPort, EventStore, Recorded};

#[derive(Debug, PartialEq, Eq)]
pub enum IngestError<S, D> {
    Store(S),
    Destination(D),
}

pub fn ingest_inbound_text<S: EventStore, D: DestinationPort>(
    store: &mut S,
    destination: &mut D,
    message: &InboundText,
) -> Result<(), IngestError<S::Error, D::Error>> {
    let recorded = store.record_inbound(message).map_err(IngestError::Store)?;
    if recorded == Recorded::Duplicate {
        return Ok(());
    }
    destination
        .deliver(message)
        .map_err(IngestError::Destination)
}
