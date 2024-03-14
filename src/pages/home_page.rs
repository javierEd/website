use leptos::*;

use crate::layouts::MainLayout;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <MainLayout title="Home">
            <h1 class="title has-text-centered">"Welcome to my personal website"</h1>

        </MainLayout>
    }
}
