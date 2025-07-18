pub mod app_err_uc;
pub mod domain;
pub mod dtos;
pub mod ui;
// TODO: to be deleted
pub mod server_fns_todo;

#[cfg(feature = "ssr")]
pub mod server;

#[cfg_attr(feature = "csr", wasm_bindgen::prelude::wasm_bindgen)]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(ui::comps::App);
}
