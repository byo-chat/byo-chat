use std::path::Path;

use byo_domain::message::InboundText;
use byo_domain::ports::{EventStore, Recorded};
use rusqlite::Connection;

#[derive(Debug)]
pub struct StoreError(rusqlite::Error);

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "event store: {}", self.0)
    }
}

impl std::error::Error for StoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<rusqlite::Error> for StoreError {
    fn from(error: rusqlite::Error) -> Self {
        Self(error)
    }
}

#[derive(Debug)]
pub struct SqliteEventStore {
    connection: Connection,
}

impl SqliteEventStore {
    pub fn open(path: &Path) -> Result<Self, StoreError> {
        let connection = Connection::open(path)?;
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS inbound (
                account TEXT NOT NULL,
                conversation TEXT NOT NULL,
                network_id TEXT NOT NULL,
                sender TEXT NOT NULL,
                text TEXT NOT NULL,
                PRIMARY KEY (account, network_id)
            );",
        )?;
        Ok(Self { connection })
    }
}

impl EventStore for SqliteEventStore {
    type Error = StoreError;

    fn record_inbound(&mut self, message: &InboundText) -> Result<Recorded, StoreError> {
        let inserted = self.connection.execute(
            "INSERT OR IGNORE INTO inbound (account, conversation, network_id, sender, text)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &message.account.0,
                &message.conversation.0,
                &message.network_id.0,
                &message.sender,
                &message.text,
            ),
        )?;
        Ok(if inserted == 0 {
            Recorded::Duplicate
        } else {
            Recorded::New
        })
    }
}
