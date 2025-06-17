use crate::ui::styles;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;

#[component]
pub fn Login() -> impl IntoView {
    let username = RwSignal::new("".to_string());
    let password = RwSignal::new("".to_string());

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
                            spawn_local(async {
                                match login(username, password).await {
                                    Ok(res) => log!("login response: '{}'.", res),
                                    Err(e) => log!("login error: '{}'.", e),
                                }
                            });
                        }
                        class="bg-green-100 hover:bg-green-200 drop-shadow-sm px-4 py-1 rounded-md">
                        Login
                    </button>
                </div>
            </div>
        </div>
    }
}

#[server]
pub async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    //
    use crate::server::AuthSession;

    log::info!("login username: '{}', password: '{}'", username, password);
    let sess: AuthSession = leptos_axum::extract().await?;
    log::info!("state: '{:#?}'", sess);
    Ok("ok".to_string())
}
