mod admin_layout;
mod admin_router;
mod components;
mod pages;
mod server_functions;

#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
mod variables;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(admin_router::AdminRouter);
}
