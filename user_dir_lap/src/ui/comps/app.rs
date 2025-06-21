use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use reactive_stores::Store;

use crate::ui::{
    comps::Navbar,
    state::UiState,
    views::{Home, Login},
};

#[component]
pub fn App() -> impl IntoView {
    //
    provide_context(Store::new(UiState::default()));

    view! {
        <Router>
            <div class="flex flex-col min-h-screen bg-gray-100">
                <Navbar/>
                <div class="flex flex-col min-h-screen justify-center items-center drop-shadow-2xl">
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/login") view=Login/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}
