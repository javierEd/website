use leptos::*;
use leptos_bulma::elements::{BBlock, BTitle};
use leptos_router::use_params_map;

use website_core::models::Post;

use crate::components::{PageTitle, TimeAgo};
use crate::i18n::{t, use_i18n};

#[server]
pub async fn get_post(slug: String) -> Result<Option<Post>, ServerFnError> {
    Ok(Post::get_by_slug(&slug).ok())
}

#[component]
pub fn ShowPostPage() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap());
    let resource = create_blocking_resource(slug, get_post);

    view! {
        <PageTitle text="Blog"/>

        <Suspense>
            {resource
                .get()
                .and_then(|data| data.ok())
                .and_then(|data| data)
                .map(|post| {
                    let title = post.title.clone();
                    let published_at_formatted = post.published_at_formatted().unwrap();
                    let published_time_ago = post.published_time_ago().unwrap();
                    let content_parser = pulldown_cmark::Parser::new(&post.content);
                    let mut content_html = String::new();
                    pulldown_cmark::html::push_html(&mut content_html, content_parser);
                    view! {
                        <PageTitle text=format!("Blog / {}", title)/>

                        <BTitle is=1>{title}</BTitle>

                        <BBlock class="has-text-right">
                            {t!(i18n, published)} " "
                            <TimeAgo
                                formatted=published_at_formatted
                                count=published_time_ago.0
                                unit=published_time_ago.1
                            />
                        </BBlock>

                        <div class="block" inner_html=content_html></div>
                    }
                })}

        </Suspense>
    }
}
