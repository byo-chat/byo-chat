#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountState {
    New,
    Authenticating,
    Connected,
    Reconnecting,
    Disconnected,
    Expired,
    Paused,
}

impl AccountState {
    #[must_use]
    pub const fn needs_login(self) -> bool {
        matches!(self, Self::New | Self::Expired)
    }
}

#[cfg(test)]
mod tests {
    use super::AccountState;

    #[test]
    fn only_new_and_expired_accounts_need_login() {
        let needing: Vec<AccountState> = [
            AccountState::New,
            AccountState::Authenticating,
            AccountState::Connected,
            AccountState::Reconnecting,
            AccountState::Disconnected,
            AccountState::Expired,
            AccountState::Paused,
        ]
        .into_iter()
        .filter(|state| state.needs_login())
        .collect();
        assert_eq!(needing, [AccountState::New, AccountState::Expired]);
    }
}
