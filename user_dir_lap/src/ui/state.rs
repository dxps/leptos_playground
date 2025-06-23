use crate::domain::model::UserAccount;
use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
pub struct UiState {
    /// Tells if the UI state has been completely initialized or not.
    /// Aka whether it is known whether the user is logged in or not.
    pub is_inited: bool,
    pub is_logged_in: bool,
    pub account: Option<UserAccount>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            is_inited: false,
            is_logged_in: false,
            account: None,
        }
    }
}
