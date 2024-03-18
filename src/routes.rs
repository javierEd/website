use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::*;

#[component]
pub fn AppRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        <Link rel="icon" href="/images/favicon.png"/>

        <Title text="Javier E."/>

        <Router>
            <Routes>
                <Route path="" view=HomePage/>
                <Route path="/about" view=AboutPage/>
                <Route path="/*any" view=NotFoundPage/>
            </Routes>
        </Router>
    }
}
