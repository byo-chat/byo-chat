#![expect(
    clippy::tests_outside_test_module,
    reason = "an integration test crate is the test module"
)]

use std::convert::Infallible;

use byo_app::ingest::ingest_inbound_text;
use byo_app::outbound::{deliver_pending, submit_reply};
use byo_domain::message::{
    AccountId, ConversationId, InboundText, NetworkMessageId, OutboundId, OutboundText,
};
use byo_domain::ports::{ConnectorPort, DestinationPort};
use byo_store::SqliteEventStore;

#[derive(Debug, Default)]
struct RecordingDestination {
    delivered: Vec<InboundText>,
}

impl DestinationPort for RecordingDestination {
    type Error = Infallible;

    fn deliver(&mut self, message: &InboundText) -> Result<(), Infallible> {
        self.delivered.push(message.clone());
        Ok(())
    }
}

fn hello_from_maria() -> InboundText {
    InboundText {
        account: AccountId(String::from("wa-1")),
        conversation: ConversationId(String::from("5571999990000@s.whatsapp.net")),
        network_id: NetworkMessageId(String::from("3EB0C767D26A1D4F")),
        sender: String::from("Maria"),
        text: String::from("Oi, tudo bem?"),
    }
}

#[test]
fn inbound_text_reaches_the_destination() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = SqliteEventStore::open(&dir.path().join("history.db")).unwrap();
    let mut destination = RecordingDestination::default();

    ingest_inbound_text(&mut store, &mut destination, &hello_from_maria()).unwrap();

    assert_eq!(destination.delivered, vec![hello_from_maria()]);
}

#[test]
fn a_message_received_twice_reaches_the_destination_once() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = SqliteEventStore::open(&dir.path().join("history.db")).unwrap();
    let mut destination = RecordingDestination::default();

    let first = ingest_inbound_text(&mut store, &mut destination, &hello_from_maria());
    let second = ingest_inbound_text(&mut store, &mut destination, &hello_from_maria());

    assert!(first.is_ok() && second.is_ok(), "{first:?} {second:?}");
    assert_eq!(destination.delivered, vec![hello_from_maria()]);
}

#[derive(Debug, Default)]
struct RecordingConnector {
    sent: Vec<OutboundText>,
}

impl ConnectorPort for RecordingConnector {
    type Error = Infallible;

    fn send(&mut self, message: &OutboundText) -> Result<(), Infallible> {
        self.sent.push(message.clone());
        Ok(())
    }
}

fn reply_to_maria() -> OutboundText {
    OutboundText {
        id: OutboundId(String::from("cw-msg-42")),
        account: AccountId(String::from("wa-1")),
        conversation: ConversationId(String::from("5571999990000@s.whatsapp.net")),
        text: String::from("Tudo ótimo, como posso ajudar?"),
    }
}

#[test]
fn a_reply_reaches_the_connector() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = SqliteEventStore::open(&dir.path().join("history.db")).unwrap();
    let mut connector = RecordingConnector::default();

    submit_reply(&mut store, &reply_to_maria()).unwrap();
    deliver_pending(&mut store, &mut connector, &AccountId(String::from("wa-1"))).unwrap();

    assert_eq!(connector.sent, vec![reply_to_maria()]);
}
