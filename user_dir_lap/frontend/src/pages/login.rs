use leptos::prelude::*;
use leptos::{html::Input, reactive::spawn_local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct LoginResponse {
    success: bool,
    message: String,
}

#[component]
fn LoginPage() -> impl IntoView {
    let username = NodeRef::<Input>::new();
    let password = NodeRef::<Input>::new();
    let message = RwSignal::new(String::new());

    let submit = move |_| {
        let username_value = username.get().unwrap().value();
        let password_value = password.get().unwrap().value();
        let login_payload = LoginRequest {
            username: username_value,
            password: password_value,
        };

        spawn_local(async move {
            let resp = reqwasm::http::Request::post("/api/login")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&login_payload).unwrap())
                .send()
                .await;

            match resp {
                Ok(response) => {
                    if let Ok(login_response) = response.json::<LoginResponse>().await {
                        message.set(login_response.message);
                    } else {
                        message.set("Invalid server response".into());
                    }
                }
                Err(_) => message.set("Login failed".into()),
            }
        });
    };

    view! {
        <div class="login-form">
            <h2>Login</h2>
            <input type="text" placeholder="Username" node_ref=username />
            <input type="password" placeholder="Password" node_ref=password />
            <button on:click=submit>Login</button>
            <p>{move || message.get()}</p>
        </div>
    }
}
