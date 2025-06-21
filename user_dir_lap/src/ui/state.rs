use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
pub struct UiState {
    pub is_logged_in: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            is_logged_in: false,
        }
    }
}
