use leptos::*;

use crate::layouts::{JobTitlesCarousel, MainLayout};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <MainLayout title="Home">
            <h2 class="title">
                "Hi, I'm Javier E."
            </h2>
            <h3 class="subtitle">
                "A "
                <JobTitlesCarousel class="has-text-centered has-text-weight-bold"/>
                " with some years of experience."
            </h3>
        </MainLayout>
    }
}
