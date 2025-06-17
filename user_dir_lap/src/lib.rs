pub mod domain;
pub mod errors;
pub mod server_fns_todo;
pub mod ui;

#[cfg(feature = "ssr")]
pub mod server;

#[cfg_attr(feature = "csr", wasm_bindgen::prelude::wasm_bindgen)]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(ui::comps::App);
}
