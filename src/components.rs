use leptos::html::Span;
use leptos::*;
use leptos_bulma::elements::{BBox, BTitle};
use leptos_meta::Title;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

use website_core::models::Post;

use crate::i18n::{t, use_i18n};

#[derive(Clone)]
pub struct JobTitle(pub ReadSignal<&'static str>);

#[component]
pub fn JobTitlesCarousel(#[prop(default = "")] class: &'static str) -> impl IntoView {
    let job_title = use_context::<JobTitle>().unwrap();
    let job_titles = create_rw_signal(vec![job_title.0.get()]);

    let node_ref = create_node_ref::<Span>();

    create_effect(move |_| {
        let mut new_job_titles = job_titles.get();

        new_job_titles.push(job_title.0.get());

        if new_job_titles.len() > 2 {
            let _ = new_job_titles.remove(0);
        }

        job_titles.set(new_job_titles);

        if let Some(element) = node_ref.get() {
            let _ = element.remove_attribute("style");
            element.set_scroll_top(0);

            if let Some(last_child) = element.child_nodes().get(1).unwrap().dyn_ref::<HtmlDivElement>() {
                let last_child_width = last_child.client_width();
                let last_child_height = last_child.client_height();
                let _ = element.set_attribute(
                    "style",
                    &format!(
                        "scroll-behavior: smooth; max-width: {}px; max-height: {}px",
                        last_child_width, last_child_height
                    ),
                );
            }

            element.set_scroll_top(element.scroll_height());
        }
    });

    view! {
        <span class=format!("job-titles-carousel {}", class) node_ref=node_ref>
            <For each=move || job_titles.get() key=|jt| jt.to_owned() children=move |jt| view! { <div>{jt}</div> }/>
        </span>
    }
}

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    let job_title = use_context::<JobTitle>().unwrap();

    move || view! { <Title text=format!("{} | Javier E. - {}", text.get(), job_title.0.get())/> }
}

#[component]
pub fn PostBoxes(posts: Vec<Post>) -> impl IntoView {
    view! {
        <For each=move || posts.clone() key=|post| post.id let:post>
            <PostBox post=post/>
        </For>
    }
}

#[component]
fn PostBox(post: Post) -> impl IntoView {
    let i18n = use_i18n();
    let published_at_formatted = post.published_at_formatted().unwrap();
    let published_time_ago = post.published_time_ago().unwrap();

    view! {
        <a href=format!("/blog/{}", post.slug)>
            <BBox class="mb-4">
                <BTitle is=3>{post.title}</BTitle>

                <div class="has-text-right">
                    {t!(i18n, published)} " "
                    <TimeAgo formatted=published_at_formatted count=published_time_ago.0 unit=published_time_ago.1/>
                </div>
            </BBox>
        </a>
    }
}

#[component]
pub fn TimeAgo(#[prop(into)] formatted: TextProp, count: i32, unit: &'static str) -> impl IntoView {
    let i18n = use_i18n();
    let count = move || count;
    let t_time_ago = move || match unit {
        "secs" => t!(i18n, seconds_ago, count = count).into_view(),
        "mins" => t!(i18n, minutes_ago, count = count).into_view(),
        "hours" => t!(i18n, hours_ago, count = count).into_view(),
        "days" => t!(i18n, days_ago, count = count).into_view(),
        "months" => t!(i18n, months_ago, count = count).into_view(),
        _ => t!(i18n, years_ago, count = count).into_view(),
    };

    view! {
        <time datetime=formatted.clone() title=formatted>
            {t_time_ago}
        </time>
    }
}
