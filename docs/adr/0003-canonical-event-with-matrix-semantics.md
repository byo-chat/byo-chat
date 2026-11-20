# Own canonical event schema with Matrix semantics; Chatwoot is just an adapter

The Bridge's internal format is an own versioned schema (`byo.*.v1`) following Matrix event semantics (message, reaction, edit, redaction, receipt, reply, group membership). Chatwoot's protocol covers neither groups, reactions nor edits; using it as the model would tie the domain to the poorest destination, so it becomes an outbound adapter. WhatsApp (wacli) and Telegram define minimum coverage.

## Considered Options

- Chatwoot's model: rejected, insufficient coverage.
- Full Matrix (every Connector as a bridge, byo-chat as a Matrix client): rejected, it would require Conduit always running and writing bridges for wacli/Telegram. Compatible semantics keep that door open.
