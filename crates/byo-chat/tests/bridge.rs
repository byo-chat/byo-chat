#![expect(
    clippy::tests_outside_test_module,
    reason = "an integration test crate is the test module"
)]

use std::cell::Cell;
use std::convert::Infallible;

use byo_app::ingest::ingest_inbound_text;
use byo_app::outbound::{DEFAULT_DEADLINE_SECONDS, DeliveryPolicy, deliver_pending, submit_reply};
use byo_domain::message::{
    AccountId, ConversationId, InboundText, NetworkMessageId, OutboundId, OutboundText, Timestamp,
};
use byo_domain::ports::{Clock, ConnectorPort, DestinationPort};
use byo_store::SqliteEventStore;

#[derive(Debug, Default)]
struct ManualClock(Cell<i64>);

impl Clock for ManualClock {
    fn now(&self) -> Timestamp {
        Timestamp(self.0.get())
    }
}

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
    let clock = ManualClock::default();
    let mut connector = RecordingConnector::default();

    submit_reply(&mut store, &clock, &reply_to_maria()).unwrap();
    deliver_pending(
        &mut store,
        &mut connector,
        &clock,
        &AccountId(String::from("wa-1")),
        DeliveryPolicy::default(),
    )
    .unwrap();

    assert_eq!(connector.sent, vec![reply_to_maria()]);
}

#[test]
fn a_reply_is_sent_once_when_delivery_runs_twice() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = SqliteEventStore::open(&dir.path().join("history.db")).unwrap();
    let clock = ManualClock::default();
    let mut connector = RecordingConnector::default();
    let account = AccountId(String::from("wa-1"));

    submit_reply(&mut store, &clock, &reply_to_maria()).unwrap();
    deliver_pending(
        &mut store,
        &mut connector,
        &clock,
        &account,
        DeliveryPolicy::default(),
    )
    .unwrap();
    deliver_pending(
        &mut store,
        &mut connector,
        &clock,
        &account,
        DeliveryPolicy::default(),
    )
    .unwrap();

    assert_eq!(connector.sent, vec![reply_to_maria()]);
}

#[test]
fn a_reply_submitted_twice_is_sent_once() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = SqliteEventStore::open(&dir.path().join("history.db")).unwrap();
    let clock = ManualClock::default();
    let mut connector = RecordingConnector::default();

    submit_reply(&mut store, &clock, &reply_to_maria()).unwrap();
    submit_reply(&mut store, &clock, &reply_to_maria()).unwrap();
    deliver_pending(
        &mut store,
        &mut connector,
        &clock,
        &AccountId(String::from("wa-1")),
        DeliveryPolicy::default(),
    )
    .unwrap();

    assert_eq!(connector.sent, vec![reply_to_maria()]);
}

#[derive(Debug, Default)]
struct ConnectorFailing {
    failing: Option<OutboundId>,
    sent: Vec<OutboundText>,
}

impl ConnectorPort for ConnectorFailing {
    type Error = String;

    fn send(&mut self, message: &OutboundText) -> Result<(), String> {
        if self.failing.as_ref() == Some(&message.id) {
            return Err(String::from("network timeout"));
        }
        self.sent.push(message.clone());
        Ok(())
    }
}

fn reply(id: &str, conversation: &str, text: &str) -> OutboundText {
    OutboundText {
        id: OutboundId(String::from(id)),
        account: AccountId(String::from("wa-1")),
        conversation: ConversationId(String::from(conversation)),
        text: String::from(text),
    }
}

#[test]
fn a_failing_conversation_holds_its_order_without_blocking_others() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = SqliteEventStore::open(&dir.path().join("history.db")).unwrap();
    let account = AccountId(String::from("wa-1"));
    let clock = ManualClock::default();
    let maria_1 = reply("m1", "maria", "first");
    let maria_2 = reply("m2", "maria", "second");
    let joao_1 = reply("j1", "joao", "hello");
    for message in [&maria_1, &maria_2, &joao_1] {
        submit_reply(&mut store, &clock, message).unwrap();
    }
    let mut connector = ConnectorFailing {
        failing: Some(OutboundId(String::from("m1"))),
        ..ConnectorFailing::default()
    };

    deliver_pending(
        &mut store,
        &mut connector,
        &clock,
        &account,
        DeliveryPolicy::default(),
    )
    .unwrap();
    let while_down = connector.sent.clone();
    connector.failing = None;
    deliver_pending(
        &mut store,
        &mut connector,
        &clock,
        &account,
        DeliveryPolicy::default(),
    )
    .unwrap();

    assert_eq!(while_down, vec![joao_1.clone()]);
    assert_eq!(connector.sent, vec![joao_1, maria_1, maria_2]);
}

#[test]
fn a_reply_past_its_deadline_becomes_a_dead_letter() {
    let dir = tempfile::tempdir().unwrap();
    let mut store = SqliteEventStore::open(&dir.path().join("history.db")).unwrap();
    let clock = ManualClock::default();
    let account = AccountId(String::from("wa-1"));
    let policy = DeliveryPolicy::default();
    let mut connector = ConnectorFailing {
        failing: Some(OutboundId(String::from("cw-msg-42"))),
        ..ConnectorFailing::default()
    };
    submit_reply(&mut store, &clock, &reply_to_maria()).unwrap();

    clock.0.set(DEFAULT_DEADLINE_SECONDS);
    let at_deadline =
        deliver_pending(&mut store, &mut connector, &clock, &account, policy).unwrap();
    clock.0.set(DEFAULT_DEADLINE_SECONDS + 1);
    let past = deliver_pending(&mut store, &mut connector, &clock, &account, policy).unwrap();
    connector.failing = None;
    let after = deliver_pending(&mut store, &mut connector, &clock, &account, policy).unwrap();

    assert_eq!(at_deadline.failed.len(), 1);
    assert_eq!(
        past.dead_lettered,
        vec![OutboundId(String::from("cw-msg-42"))]
    );
    assert_eq!(
        (after.sent, after.dead_lettered, connector.sent),
        (Vec::new(), Vec::new(), Vec::new())
    );
}
