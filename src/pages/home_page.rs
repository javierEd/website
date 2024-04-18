use leptos::*;
use leptos_bulma::elements::{BBlock, BSubtitle, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{JobTitlesCarousel, PageTitle, PostBoxes};
use crate::i18n::{t, use_i18n};
use crate::server_functions::get_posts;

#[component]
pub fn HomePage() -> impl IntoView {
    let i18n = use_i18n();
    let resource = create_resource(move || 1, get_posts);

    view! {
        <PageTitle text=t!(i18n, home)/>

        <BTitle is=1>{t!(i18n, hi_im)} " Javier E."</BTitle>

        <BSubtitle is=3>
            {t!(i18n, a)} " " <JobTitlesCarousel class="has-text-centered has-text-weight-bold"/> " "
            {t!(i18n, with_some_years_of_experience)}
        </BSubtitle>

        <BSection>
            <BTitle is=2>{t!(i18n, latest_posts)}</BTitle>

            <Suspense>
                {resource
                    .get()
                    .and_then(|v| v.ok())
                    .map(|(posts, _)| {
                        if !posts.is_empty() {
                            view! {
                                <PostBoxes posts=posts/>

                                <a class="button is-fullwidth" href="/blog">
                                    {t!(i18n, show_more)}
                                </a>
                            }
                                .into_view()
                        } else {
                            view! { <BBlock class="has-text-centered">{t!(i18n, nothing_to_see_here_yet)}</BBlock> }
                        }
                    })}

            </Suspense>
        </BSection>
    }
}
