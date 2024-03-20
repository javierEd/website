use leptos::*;
use leptos_i18n::Locale;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::utils::FromToStringCodec;
use leptos_use::{use_cookie_with_options, UseCookieOptions};

use crate::i18n::provide_i18n_context;
use crate::layouts::MainLayout;
use crate::pages::*;

#[component]
pub fn AppRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let i18n = provide_i18n_context();

    let (locale_cookie, _) = use_cookie_with_options::<String, FromToStringCodec>(
        "i18n_pref_locale",
        UseCookieOptions::default()
            .ssr_cookies_header_getter(|| {
                #[cfg(feature = "ssr")]
                {
                    use_context::<leptos_spin::RequestParts>().and_then(|req| {
                        req.headers()
                            .iter()
                            .find(|(key, _)| key == "cookie")
                            .and_then(|(_, value)| String::from_utf8(value.to_vec()).ok())
                    })
                }
                #[cfg(not(feature = "ssr"))]
                {
                    None
                }
            })
            .ssr_set_cookie(|_| {}),
    );

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
