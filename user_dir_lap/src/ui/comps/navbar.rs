use crate::ui::{
    comps::NavUserMenu,
    logic::get_current_user,
    state::{UiState, UiStateStoreFields},
    styles::{self, NAVBAR_LIST, NAVBAR_LIST_ITEM},
};
use leptos::prelude::*;
use leptos::{logging::log, reactive::spawn_local};
use reactive_stores::Store;

#[component]
pub fn Navbar() -> impl IntoView {
    //
    let state = expect_context::<Store<UiState>>();

    Effect::new(move || {
        log!("After load, checking the ui state ...");
        if !state.is_inited().get() {
            log!("Initing the ui state ...");
            spawn_local(async move {
                _ = get_current_user()
                    .await
                    .map(|res| match res {
                        Some(account) => {
                            log!("Got current user account: {account:?}");
                            state.is_logged_in().set(true);
                            state.account().set(Some(account));
                        }
                        None => {
                            log!("No current user account found.");
                            state.is_logged_in().set(false);
                            state.account().set(None);
                        }
                    })
                    .map_err(|err| {
                        log!("Failed to get the current user. Error: {}", err);
                    });
            });
            state.is_inited().set(true);
        }
    });

    view! {
        <nav class="absolute w-full px-4 py-1 flex justify-between items-center bg-white z-40">
            <a href="/" class="py-1.5 hover:bg-white">
                <img src="/img/favicon/favicon-32x32.png" class="w-[24px] h-[24px]"/>
            </a>
            <Show
                when=move || state.is_logged_in().get()
                fallback=|| view!{
                    <a href="/login"
                    class=styles::NAVBAR_LINK
                    >Login</a>
                }>
                <ul class=NAVBAR_LIST>
                    <li>
                        <a href="/" class=NAVBAR_LIST_ITEM>Home</a>
                    </li>
                    <li class="text-gray-400">
                        <NavSep/>
                    </li>
                    <li>
                        <a href="/accounts" class=NAVBAR_LIST_ITEM>Accounts</a>
                    </li>
                </ul>
                <NavUserMenu/>
            </Show>
        </nav>
    }
}

#[component]
pub fn NavSep() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor"
            class="w-4 h-4 current-fill" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M12 5v0m0 7v0m0 7v0m0-13a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"
            />
        </svg>
    }
}
