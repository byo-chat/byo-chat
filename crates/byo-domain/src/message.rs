#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkMessageId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingOutbound {
    pub message: OutboundText,
    pub enqueued_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboundId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboundText {
    pub id: OutboundId,
    pub account: AccountId,
    pub conversation: ConversationId,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundText {
    pub account: AccountId,
    pub conversation: ConversationId,
    pub network_id: NetworkMessageId,
    pub sender: String,
    pub text: String,
}
