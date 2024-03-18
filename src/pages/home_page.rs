use leptos::*;

use crate::layouts::{JobTitlesCarousel, MainLayout};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <MainLayout title="Home">
            <h2 class="title is-1">
                "Hi, I'm Javier E."
            </h2>
            <h3 class="subtitle is-3">
                "A "
                <JobTitlesCarousel class="has-text-centered has-text-weight-bold"/>
                " with some years of experience."
            </h3>
        </MainLayout>
    }
}
