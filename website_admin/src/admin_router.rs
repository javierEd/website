use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::*;

#[component]
pub fn AdminRouter() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/admin/pkg/website_admin.css"/>

        <Link rel="icon" href="/images/favicon.png"/>

        <Title text="Javier E. - Admin Panel"/>

        <Router>
            <Routes>
                <Route path="/admin" view=HomePage/>
                <Route path="/admin/login" view=LoginPage/>
                <Route path="/admin/posts" view=PostsPage/>
                <Route path="/admin/posts/new" view=NewPostPage/>
                <Route path="/admin/posts/:id/edit" view=EditPostPage/>
                <Route path="/admin/*any" view=NotFoundPage/>
            </Routes>
        </Router>
    }
}
