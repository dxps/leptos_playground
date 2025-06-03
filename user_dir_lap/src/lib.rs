pub mod components;
pub mod errors;
pub mod todo_server_fns;

#[cfg(feature = "ssr")]
pub mod fallback;

#[cfg_attr(feature = "csr", wasm_bindgen::prelude::wasm_bindgen)]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(components::App);
}

#[cfg(feature = "ssr")]
pub mod ssr {
    // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:iam.db").await?)
    }
}
