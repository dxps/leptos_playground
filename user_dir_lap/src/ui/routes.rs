use crate::ui::views::{Home, Login, Logout, UserAccounts};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Routes},
    path,
};

pub fn routes() -> impl IntoView {
    view! {
        <Routes fallback=|| "Not found.">
            <Route path=path!("/") view=Home/>
            <Route path=path!("/login") view=Login/>
            <Route path=path!("/logout") view=Logout/>
            <Route path=path!("/accounts") view=UserAccounts/>
        </Routes>
    }
}
