use leptos::*;

use crate::components::{JobTitlesCarousel, PageTitle};
use crate::i18n::{t, use_i18n};

#[component]
pub fn HomePage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <PageTitle text=t!(i18n, home)/>

        <h2 class="title is-1">{t!(i18n, hi_im)}" Javier E."</h2>

        <h3 class="subtitle is-3">
            {t!(i18n, a)}
            " "
            <JobTitlesCarousel class="has-text-centered has-text-weight-bold"/>
            " "
            {t!(i18n, with_some_years_of_experience)}
        </h3>
    }
}
