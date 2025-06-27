use crate::ui::logic::login;
use crate::ui::state::{UiState, UiStateStoreFields};
use crate::ui::styles;
use leptos::logging::log;
use leptos::reactive::spawn_local;
use leptos::{html, prelude::*};
use leptos_router::NavigateOptions;
use reactive_stores::Store;

#[component]
pub fn Login() -> impl IntoView {
    //
    let username_elem: NodeRef<html::Input> = NodeRef::new();

    let username = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());
    let login_err: RwSignal<Option<String>> = RwSignal::new(None);

    let hide_login_err = move || login_err.get().is_none();
    let login_err_txt = move || match login_err.get() {
        Some(err) => {
            if err.contains("wrong credentials") {
                "Wrong credentials.".to_string()
            } else {
                err.to_string()
            }
        }
        None => "".to_string(),
    };
    let login_ok = RwSignal::new(false);

    let state = expect_context::<Store<UiState>>();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if login_ok.get() {
            state.is_logged_in().set(true);
            log!(
                "Updated ui state with is_logged_in: {}",
                state.is_logged_in().get()
            );
            navigate("/", NavigateOptions::default());
        }
    });

    Effect::new(move || {
        if let Some(node) = username_elem.get() {
            _ = (*node).focus();
        }
    });

    view! {
        <div class="bg-white rounded-md p-6 min-w-[350px]">
            <div class="flex">
                <p class=styles::TITLE_CSS>Login</p>
                <a href="/" class=styles::CLOSE_SYMBOL_CSS>x</a>
            </div>

            <div class="mt-8 space-y-4 text-gray-600">
                <p>Use your credentials to authenticate.</p>
                <div class="flex flex-col items-center mt-8 mb-12 space-y-4">
                    <input type="text" id="username" placeholder="Username"
                        autocomplete="off" bind:value=username
                        node_ref=username_elem
                        class="px-3 py-1 rounded-lg outline-none border-1.5 focus:border-green-300 w-64"
                    />
                    <input type="password" id="password" placeholder="Password"
                        autocomplete="off" bind:value=password
                        class="px-3 py-1 rounded-lg outline-none border-1.5 focus:border-green-300 w-64"
                    />
                    <button
                        on:click=move |_| {
                            let username = username.get().clone();
                            let password = password.get().clone();
                            _ = spawn_local(async move {
                                match login(username, password).await {
                                    Ok(login_res) => {
                                        if login_res.is_succcess {
                                            login_err.set(None);
                                            log!("Login succeeded.");
                                            login_ok.set(true);
                                        } else {
                                            let err = login_res.error.unwrap().to_string();
                                            log!("Login failed with error: '{:#?}'.", err);
                                            login_err.set(Some(err));
                                        }
                                    },
                                    Err(err) => {
                                        log!("Login failed internall with error: '{:#?}'.", err);
                                        login_err.set(Some(err.to_string()));
                                    },
                                };
                            });
                        }
                        class=styles::BUTTON_CSS>
                        Login
                    </button>
                    <p class:hidden=hide_login_err>{login_err_txt}</p>
                </div>
            </div>
        </div>
    }
}
