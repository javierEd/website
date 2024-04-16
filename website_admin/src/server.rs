use leptos_spin::server_fn::register_explicit;
use leptos_spin::{render_best_match_to_stream, RouteTable};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::http_component;

#[http_component]
async fn handle_website_admin(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();

    conf.leptos_options.site_root = "target/site-admin".to_owned();
    conf.leptos_options.site_pkg_dir = "admin/pkg".to_owned();

    register_explicit::<crate::pages::AttemptToCreatePost>();
    register_explicit::<crate::pages::AttemptToDeletePost>();
    register_explicit::<crate::pages::AttemptToUpdatePost>();
    register_explicit::<crate::pages::AttemptToLogin>();
    register_explicit::<crate::pages::GetPost>();
    register_explicit::<crate::pages::GetPosts>();
    register_explicit::<crate::admin_layout::AttemptToLogout>();
    register_explicit::<crate::server_functions::IsAdminEnabled>();
    register_explicit::<crate::server_functions::IsAuthenticated>();

    let router = crate::admin_router::AdminRouter;

    let mut routes = RouteTable::build(router);
    routes.add_server_fn_prefix("/admin/api").unwrap();

    render_best_match_to_stream(req, resp_out, &routes, router, &conf.leptos_options).await
}
