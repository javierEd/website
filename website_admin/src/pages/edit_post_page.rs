use leptos::*;
use leptos_router::{use_params_map, ActionForm};

use website_core::models::Post;

use crate::admin_layout::AdminLayout;
use crate::components::PostForm;

#[server(prefix = "/admin/api")]
pub async fn attempt_to_update_post(
    id: i16,
    title: String,
    slug: String,
    content: String,
    publish: Option<bool>,
) -> Result<bool, ServerFnError> {
    crate::server_functions::require_authentication().await?;

    let post = Post::get_by_id(id)?;

    let is_saved = post
        .update(
            title.trim(),
            &slug.trim().to_lowercase(),
            content.trim(),
            publish.unwrap_or_default(),
        )
        .is_ok();

    if is_saved {
        leptos_spin::redirect("/admin/posts");
    }

    Ok(is_saved)
}

#[server(prefix = "/admin/api")]
pub async fn get_post(id: i16) -> Result<Option<Post>, ServerFnError> {
    Ok(Post::get_by_id(id).ok())
}

#[component]
pub fn EditPostPage() -> impl IntoView {
    let params = use_params_map();
    let id = params
        .with(|params| params.get("id").cloned().unwrap())
        .parse::<i16>()
        .unwrap();
    let resource = create_blocking_resource(|| (), move |_| get_post(id));
    let server_action = create_server_action::<AttemptToUpdatePost>();
    let action_value = server_action.value();

    view! {
        <AdminLayout title="Edit Post">
            <h2 class="title is-1">"Edit Post"</h2>

            <Suspense>
                {resource
                    .get()
                    .and_then(|data| data.ok())
                    .and_then(|data| data)
                    .map(|post| {
                        view! {
                            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true">
                                <PostForm
                                    action_value=action_value
                                    is_pending=server_action.pending()
                                    id=post.id.to_string()
                                    title=post.title
                                    slug=post.slug
                                    content=post.content
                                    publish=post.published_at.is_some()
                                />
                            </ActionForm>
                        }
                    })}

            </Suspense>
        </AdminLayout>
    }
}
