mod components;
mod main_layout;
mod main_router;
mod pages;
mod server_functions;

#[cfg(feature = "ssr")]
mod server;

leptos_i18n::load_locales!();

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(main_router::MainRouter);
}
