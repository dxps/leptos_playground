use crate::ui::{
    comps::NavUserMenu,
    logic::get_current_user,
    state::{UiState, UiStateStoreFields},
    styles,
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
        <ul
            class="hidden absolute top-1/2 sm:left-1/3 sm:pl-16 md:left-1/2 lg:left-1/2
                    transform -translate-y-1/2 -translate-x-1/2"
        >
            <li>
                <a href="/" class="text-sm text-gray-600 py-1 px-4 hover:bg-gray-100 rounded-lg transition duration-200">Home</a>
            </li>
        </ul>
        <Show
        when=move || state.is_logged_in().get()
        fallback=|| view!{
            <a href="/login"
               class=styles::NAVBAR_LINK
               >Login</a>
        }>
           <NavUserMenu/>
        </Show>
    </nav>
    }
}
