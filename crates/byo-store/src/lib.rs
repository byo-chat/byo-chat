use std::path::Path;

use byo_domain::message::{AccountId, ConversationId, InboundText, OutboundId, OutboundText};
use byo_domain::ports::{EventStore, OutboundQueue, Recorded};
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
            );
            CREATE TABLE IF NOT EXISTS outbound (
                seq INTEGER PRIMARY KEY AUTOINCREMENT,
                id TEXT NOT NULL UNIQUE,
                account TEXT NOT NULL,
                conversation TEXT NOT NULL,
                text TEXT NOT NULL,
                sent INTEGER NOT NULL DEFAULT 0
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
        Ok(recorded(inserted))
    }
}

fn recorded(inserted: usize) -> Recorded {
    if inserted == 0 {
        Recorded::Duplicate
    } else {
        Recorded::New
    }
}

impl OutboundQueue for SqliteEventStore {
    type Error = StoreError;

    fn enqueue(&mut self, message: &OutboundText) -> Result<Recorded, StoreError> {
        let inserted = self.connection.execute(
            "INSERT OR IGNORE INTO outbound (id, account, conversation, text)
             VALUES (?1, ?2, ?3, ?4)",
            (
                &message.id.0,
                &message.account.0,
                &message.conversation.0,
                &message.text,
            ),
        )?;
        Ok(recorded(inserted))
    }

    fn pending(&mut self, account: &AccountId) -> Result<Vec<OutboundText>, StoreError> {
        let mut statement = self.connection.prepare(
            "SELECT id, conversation, text FROM outbound
             WHERE account = ?1 AND sent = 0 ORDER BY seq",
        )?;
        let rows = statement.query_map([&account.0], |row| {
            Ok(OutboundText {
                id: OutboundId(row.get(0)?),
                account: account.clone(),
                conversation: ConversationId(row.get(1)?),
                text: row.get(2)?,
            })
        })?;
        Ok(rows.collect::<Result<_, _>>()?)
    }

    fn mark_sent(&mut self, id: &OutboundId) -> Result<(), StoreError> {
        self.connection
            .execute("UPDATE outbound SET sent = 1 WHERE id = ?1", [&id.0])?;
        Ok(())
    }
}
