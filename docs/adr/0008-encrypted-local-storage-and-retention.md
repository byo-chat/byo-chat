# Encrypted local storage, retention and local redaction

Because byo-chat keeps a copy of personal messages (ADR-0007), the SQLite database is encrypted with SQLCipher, keyed from the OS keychain (env var or 0600 key file on headless Linux). Media is stored per Account, content-addressed by hash and encrypted, with only references in SQLite. Retention is configurable per Account: text defaults to forever, local media to 90 days (afterwards only the remote reference is kept). When a Network deletes a message, the event is redacted and its local content erased, keeping only metadata (who, when, type), as Matrix does. `byo-chat accounts export` exports an Account's history; removing an Account erases its history and media after confirmation.

## Considered Options

- Keeping deleted content marked "deleted by sender": rejected in favour of privacy and honouring the sender's intent.
- Plain SQLite with OS disk encryption only: rejected — the database would be readable by any process of the user.
