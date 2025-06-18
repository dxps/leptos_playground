use crate::dtos::LoginResult;
use crate::ui::styles;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;

#[component]
pub fn Login() -> impl IntoView {
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
                        autofocus autocomplete="off" bind:value=username
                        class="px-3 py-1 rounded-lg outline-none border-1.5 focus:border-green-300 w-64"
                    />
                    <input type="password" id="password" placeholder="Password"
                        autofocus autocomplete="off" bind:value=password
                        class="px-3 py-1 rounded-lg outline-none border-1.5 focus:border-green-300 w-64"
                    />
                    <button
                        on:click=move |_| {
                            let username = username.get().clone();
                            let password = password.get().clone();
                            spawn_local(async move {
                                match login(username, password).await {
                                    Ok(login_res) => {
                                        if login_res.is_succcess {
                                            login_err.set(None);
                                            log!("Login succeeded.");
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
                        class="bg-green-100 hover:bg-green-200 drop-shadow-sm px-4 mt-6 py-1 rounded-md">
                        Login
                    </button>
                    <p class:hidden=hide_login_err>{login_err_txt}</p>
                </div>
            </div>
        </div>
    }
}

#[server]
pub async fn login(username: String, password: String) -> Result<LoginResult, ServerFnError> {
    //
    use crate::server::Session;

    let sess: Session = leptos_axum::extract().await?;

    let login_res = sess.user_mgmt.authenticate_user(username, password).await;
    if login_res.is_succcess {
        sess.auth_session
            .login_user(login_res.clone().account.unwrap().id);
    }

    Ok(login_res)
}
