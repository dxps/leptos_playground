use leptos::prelude::*;

#[component]
pub fn NavUserMenu() -> impl IntoView {
    view! {
        <a
        href="/login"
        class="text-sm text-gray-600 py-1 px-4 hover:bg-gray-100 rounded-lg transition duration-200 sm:inline-block sm:ml-auto sm:mr-3"
        >Login</a>
    }
}
