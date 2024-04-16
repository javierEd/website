use leptos::*;
use leptos_bulma::components::{BModal, BModalContent, BPagination};
use leptos_bulma::elements::{BBox, BTable, BTbody, BThead};
use leptos_router::use_query_map;
use website_core::models::Post;

use crate::admin_layout::AdminLayout;

#[server(prefix = "/admin/api")]
pub async fn get_posts(page: i16) -> Result<(Vec<Post>, i16), ServerFnError> {
    let posts = Post::paginate(page, false)?;
    let pages_count = Post::pages_count(false)?;

    Ok((posts, pages_count))
}

#[server(prefix = "/admin/api")]
pub async fn attempt_to_delete_post(id: i16) -> Result<bool, ServerFnError> {
    let post = Post::get_by_id(id)?;

    Ok(post.delete().is_ok())
}

#[component]
pub fn PostsPage() -> impl IntoView {
    let query = use_query_map();

    let current_page = move || {
        query
            .with(|q| q.get("page").cloned().unwrap_or("1".to_owned()))
            .parse::<i16>()
            .unwrap_or(1)
    };

    let page = create_rw_signal(current_page());
    let show_delete_confirmation_for = create_rw_signal(None);
    let show_delete_confirmation = create_rw_signal(false);
    let action_delete = create_action(|id: &i16| attempt_to_delete_post(*id));
    let resource = create_resource(move || page.get(), get_posts);

    let close_delete_confirmation = move |_| {
        show_delete_confirmation.set(false);
        show_delete_confirmation_for.set(None)
    };

    create_effect(move |_| page.set(current_page()));

    create_effect(move |_| {
        if Some(Ok(true)) == action_delete.value().get() {
            resource.refetch();
            show_delete_confirmation.set(false);
            show_delete_confirmation_for.set(None)
        }
    });

    view! {
        <AdminLayout title="Posts">
            <h2 class="title is-1">"Posts"</h2>

            <Suspense>
                {resource
                    .get()
                    .and_then(|v| v.ok())
                    .map(|(posts, pages_count)| {
                        view! {
                            <BTable class="is-fullwidth is-hoverable">
                                <BThead
                                    each_cell=move || [
                                        "ID",
                                        "Title",
                                        "Slug",
                                        "Published at",
                                        "Created at",
                                        "Updated at",
                                        "...",
                                    ]

                                    key=|cell| *cell
                                    let:cell
                                >
                                    {cell}
                                </BThead>
                                <BTbody each_row=move || posts.clone() key=|post| post.id let:post>
                                    {move || {
                                        let published_at_formatted = post
                                            .clone()
                                            .published_at_formatted()
                                            .unwrap_or_default();
                                        let created_at_formatted = post.clone().created_at_formatted();
                                        let updated_at_formatted = post.updated_at_formatted().unwrap_or_default();
                                        view! {
                                            <th>{post.id}</th>
                                            <td>{&post.title}</td>
                                            <td>{&post.slug}</td>
                                            <td>
                                                <time datetime=&published_at_formatted>{published_at_formatted}</time>
                                            </td>
                                            <td>
                                                <time datetime=&created_at_formatted>{created_at_formatted}</time>
                                            </td>
                                            <td>
                                                <time datetime=&updated_at_formatted>{updated_at_formatted}</time>
                                            </td>
                                            <td>
                                                <div class="buttons is-centered">
                                                    <a
                                                        class="button"
                                                        href=format!("/blog/{}", post.slug)
                                                        target="_blank"
                                                    >
                                                        "Go"
                                                    </a>
                                                    <a class="button" href=format!("/admin/posts/{}/edit", post.id)>
                                                        "Edit"
                                                    </a>
                                                    <a
                                                        class="button"
                                                        on:click=move |_| {
                                                            show_delete_confirmation_for.set(Some(post.id));
                                                            show_delete_confirmation.set(true)
                                                        }
                                                    >

                                                        "Delete"
                                                    </a>
                                                </div>
                                            </td>
                                        }
                                    }}

                                </BTbody>

                            </BTable>

                            <BPagination class="is-centered" count=pages_count/>

                            <BModal is_active=show_delete_confirmation on_close=close_delete_confirmation>
                                <BModalContent>
                                    <BBox class="has-text-centered">
                                        <h3 class="title is-4">"Are you sure you want to delete this post?"</h3>
                                        <div class="buttons is-centered">
                                            <a
                                                class="button is-primary"
                                                on:click=move |_| {
                                                    action_delete.dispatch(show_delete_confirmation_for.get().unwrap())
                                                }
                                            >

                                                "Accept"
                                            </a>
                                            <a class="button" on:click=close_delete_confirmation>
                                                "Cancel"
                                            </a>
                                        </div>
                                    </BBox>
                                </BModalContent>
                            </BModal>
                        }
                    })}

            </Suspense>
        </AdminLayout>
    }
}
