use leptos::*;
use leptos_bulma::components::BPagination;
use leptos_bulma::elements::{BBlock, BTitle};
use leptos_router::use_query_map;

use crate::components::{PageTitle, PostBoxes};
use crate::i18n::{t, use_i18n};
use crate::server_functions::get_posts;

#[component]
pub fn BlogPage() -> impl IntoView {
    let i18n = use_i18n();
    let query = use_query_map();

    let current_page = move || {
        query
            .with(|q| q.get("page").cloned().unwrap_or("1".to_owned()))
            .parse::<i16>()
            .unwrap_or(1)
    };

    let page = create_rw_signal(current_page());
    let resource = create_resource(move || page.get(), get_posts);

    create_effect(move |_| page.set(current_page()));

    view! {
        <PageTitle text="Blog"/>

        <BTitle is=1>{t!(i18n, blog)}</BTitle>

        <Suspense>
            {resource
                .get()
                .and_then(|v| v.ok())
                .map(|(posts, pages_count)| {
                    if !posts.is_empty() || pages_count > 1 {
                        view! {
                            <PostBoxes posts=posts/>

                            <BPagination class="is-centered mt-4" count=pages_count/>
                        }
                            .into_view()
                    } else {
                        view! { <BBlock class="has-text-centered">{t!(i18n, nothing_to_see_here_yet)}</BBlock> }
                    }
                })}

        </Suspense>
    }
}
