use crate::ui::{comps::Navbar, routes::routes, state::UiState};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::Router;
use reactive_stores::Store;

#[component]
pub fn App() -> impl IntoView {
    //
    provide_meta_context();
    provide_context(Store::new(UiState::default()));

    view! {
        <Title text="User Directory | A LAP stack solution" />
        <Stylesheet id="leptos" href="/pkg/user_dir_lap.css" />
        <Router>
            <div class="flex flex-col min-h-screen bg-gray-100">
                <Navbar/>
                <div class="flex flex-col min-h-screen justify-center items-center drop-shadow-2xl">
                    {routes()}
                </div>
            </div>
        </Router>
    }
}
