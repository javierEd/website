use leptos::*;

use crate::admin_layout::AdminLayout;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <AdminLayout title="Home">

            <h2 class="title is-1">"Home"</h2>
        </AdminLayout>
    }
}
