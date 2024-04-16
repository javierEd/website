use leptos::*;
use leptos_router::ActionForm;

use crate::admin_layout::AdminLayout;
use crate::components::PostForm;

#[server(prefix = "/admin/api")]
pub async fn attempt_to_create_post(
    title: String,
    slug: String,
    content: String,
    publish: Option<bool>,
) -> Result<bool, ServerFnError> {
    crate::server_functions::require_authentication().await?;

    let is_saved = website_core::models::Post::insert(
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

#[component]
pub fn NewPostPage() -> impl IntoView {
    let server_action = create_server_action::<AttemptToCreatePost>();
    let action_value = server_action.value();

    view! {
        <AdminLayout title="New Post">
            <h2 class="title is-1">"New Post"</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true">
                <PostForm action_value=action_value is_pending=server_action.pending()/>
            </ActionForm>
        </AdminLayout>
    }
}
