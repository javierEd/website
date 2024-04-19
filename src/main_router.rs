use leptos::*;
use leptos_i18n::Locale;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::utils::FromToStringCodec;
use leptos_use::{use_color_mode_with_options, use_cookie, UseColorModeOptions, UseColorModeReturn};

use crate::i18n::provide_i18n_context;
use crate::main_layout::MainLayout;
use crate::pages::*;

pub fn use_app_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(
        UseColorModeOptions::default()
            .cookie_enabled(true)
            .cookie_name("pref_color_mode")
            .attribute("data-theme")
            .emit_auto(true)
            .transition_enabled(true),
    )
}

#[component]
pub fn MainRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let i18n = provide_i18n_context();

    let (locale_cookie, _) = use_cookie::<String, FromToStringCodec>("i18n_pref_locale");

    i18n.set_locale(Locale::from_str(&locale_cookie.get().unwrap_or("en".to_owned())).unwrap_or_default());

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        <Link rel="icon" href="/images/favicon.png"/>

        <Title text="Javier E. - Software Developer"/>

        <div class="loading-overlay" class:is-done=leptos_dom::is_browser></div>

        <Router>
            <MainLayout>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/blog" view=BlogPage/>
                    <Route path="/blog/:slug" view=ShowPostPage/>
                    <Route path="/*any" view=NotFoundPage/>
                </Routes>
            </MainLayout>
        </Router>
    }
}
