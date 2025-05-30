use leptos::prelude::*;

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>Welcome to Leptos Auth</h1>
            <a href="/login">Go to Login</a>
        </div>
    }
}
