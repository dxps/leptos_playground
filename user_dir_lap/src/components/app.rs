use leptos::prelude::*;

use crate::components::Todos;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <header>
            <h1>"My Tasks"</h1>
        </header>
        <main>
            <Todos/>
        </main>
    }
}
