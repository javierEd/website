use leptos::*;

use crate::admin_layout::AdminLayout;

/// 404 - Not Found
#[component]
pub fn NotFoundPage() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_spin::ResponseOptions>();
        resp.set_status(404);
    }

    view! {
        <AdminLayout title="Error 404: Page Not Found">

            <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>
        </AdminLayout>
    }
}
