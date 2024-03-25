use leptos::*;
use leptos_i18n::Locale;
use leptos_meta::*;
use leptos_router::*;

use crate::i18n::provide_i18n_context;
use crate::main_layout::MainLayout;
use crate::pages::*;

#[cfg(feature = "ssr")]
fn use_locale_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    use leptos_use::{use_cookie_with_options, UseCookieOptions};

    use_cookie_with_options::<String, leptos_use::utils::FromToStringCodec>(
        "i18n_pref_locale",
        UseCookieOptions::default()
            .ssr_cookies_header_getter(|| {
                use_context::<leptos_spin::RequestParts>().and_then(|req| {
                    req.headers()
                        .iter()
                        .find(|(key, _)| key == "cookie")
                        .and_then(|(_, value)| String::from_utf8(value.to_vec()).ok())
                })
            })
            .ssr_set_cookie(|_| {}),
    )
}

#[cfg(not(feature = "ssr"))]
fn use_locale_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>("i18n_pref_locale")
}

#[component]
pub fn MainRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let i18n = provide_i18n_context();

    let (locale_cookie, _) = use_locale_cookie();

    i18n.set_locale(Locale::from_str(&locale_cookie.get().unwrap_or("en".to_owned())).unwrap_or_default());

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        <Link rel="icon" href="/images/favicon.png"/>

        <Title text="Javier E. - Software Developer"/>

        <Router>
            <MainLayout>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/*any" view=NotFoundPage/>
                </Routes>
            </MainLayout>
        </Router>
    }
}
