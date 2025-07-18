pub mod error_template;
pub mod errors;
#[cfg(feature = "ssr")]
pub mod fallback;
pub mod todo;

pub mod customers;
pub mod table_classes;
pub mod table_data_provider;

#[cfg(feature = "ssr")]
pub mod database;

#[cfg_attr(feature = "csr", wasm_bindgen::prelude::wasm_bindgen)]
pub fn hydrate() {
    use crate::todo::*;
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(TodoApp);
}
