use crate::ui::logic::logout;
use crate::ui::state::{UiState, UiStateStoreFields};
use crate::ui::styles;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_router::NavigateOptions;
use reactive_stores::Store;

#[component]
pub fn Logout() -> impl IntoView {
    //
    let logout_ok = RwSignal::new(false);
    let state = expect_context::<Store<UiState>>();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if logout_ok.get() {
            state.is_logged_in().set(false);
            log!("Logged out is complete.");
            navigate("/", NavigateOptions::default());
        }
    });

    view! {
        <div class="bg-white rounded-md p-6 min-w-[350px]">
            <div class="flex">
                <p class=styles::TITLE_CSS>Logout</p>
                <a href="/" class=styles::CLOSE_SYMBOL_CSS>x</a>
            </div>

            <div class="mt-8 space-y-4 text-gray-600">
                <p>You are about to logout. Hope to see you soon!</p>
                <div class="flex flex-col items-center mt-8 mb-12 space-y-4">
                    <button class=styles::BUTTON_CSS
                        on:click=move |_| {
                            spawn_local(async move {
                                if let Err(e) = logout().await {
                                    log!("Logout error: {}", e);
                                };
                                logout_ok.set(true);
                            });
                        }
                    >Logout</button>
                </div>
            </div>
        </div>
    }
}
