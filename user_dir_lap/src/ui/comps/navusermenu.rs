use leptos::prelude::*;
use reactive_stores::Store;

use crate::ui::{
    state::{UiState, UiStateStoreFields},
    styles::NAVBAR_LINK,
};

#[component]
pub fn NavUserMenu() -> impl IntoView {
    //
    let state = expect_context::<Store<UiState>>();

    let route = move || match state.is_logged_in().get() {
        true => "/logout",
        false => "/login",
    };
    let label = move || match state.is_logged_in().get() {
        true => "Logout",
        false => "Login",
    };

    view! {
        <a class=NAVBAR_LINK href={route}>{label}</a>
    }
}
