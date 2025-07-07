use leptos::prelude::*;
use reactive_stores::Store;

use crate::ui::{
    state::{UiState, UiStateStoreFields},
    styles::{NAVBAR_LINK, USER_MENU_LINK},
};

#[component]
pub fn NavUserMenu() -> impl IntoView {
    //
    let state = expect_context::<Store<UiState>>();

    view! {
        <Show when=move || state.is_logged_in().get()
            fallback=|| view! { <a class=NAVBAR_LINK href="/login">Login</a> }>
            <div
                class="text-sm text-gray-600 hover:bg-gray-100 rounded-lg transition duration-200 flex flex-col items-end overflow-visible">
                <button
                    class="px-8 py-1 align rounded-lg text-sm outline-none"
                    on:click=move |_| {
                        state.open_user_menu().set(!state.open_user_menu().get());
                    }>
                    <div class="rounded-full justify-center">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-4 h-4" viewBox="0 0 512 512">
                            <path
                                d="M337.711 241.3a16 16 0 0 0-11.461 3.988c-18.739 16.561-43.688 25.682-70.25
                                    25.682s-51.511-9.121-70.25-25.683a16.007 16.007 0 0 0-11.461-3.988c-78.926 4.274-140.752 
                                    63.672-140.752 135.224v107.152C33.537 499.293 46.9 512 63.332 512h385.336c16.429 0 29.8-12.707 
                                    29.8-28.325V376.523c-.005-71.552-61.831-130.95-140.757-135.223zM446.463 480H65.537V376.523c0-52.739 
                                    45.359-96.888 104.351-102.8C193.75 292.63 224.055 302.97 256 302.97s62.25-10.34 86.112-29.245c58.992 5.91 
                                    104.351 50.059 104.351 102.8zM256 234.375a117.188 117.188 0 1 0-117.188-117.187A117.32 
                                    117.32 0 0 0 256 234.375zM256 32a85.188 85.188 0 1 1-85.188 85.188A85.284 85.284 0 0 1 256 32z"
                                data-original="#000000"></path>
                        </svg>
                    </div>
                </button>
            </div>
            <Show when=move || state.open_user_menu().get() fallback=|| view! { <></> }>
                <NavUserDropdown />
            </Show>
        </Show>
    }
}

#[component]
pub fn NavUserDropdown() -> impl IntoView {
    //
    let state = expect_context::<Store<UiState>>();

    view! {
        <div
            style="width: 100%; height: 1000%; padding: 0; position: absolute; top: 0; left: 0;"
            on:click=move |_| state.open_user_menu().set(false)>
            <div class="w-20 mt-14 mr-[60px] bg-white rounded-lg shadow-2xl float-right">
                <div>
                    <ul class="shadow-2xl bg-white py-2 z-[1000] min-w-full w-max rounded-lg max-h-96 overflow-auto">
                        <li>
                            <a class=USER_MENU_LINK href="/profile">Profile</a>
                        </li>
                        <li>
                            <a class=USER_MENU_LINK href="/logout">Logout</a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
