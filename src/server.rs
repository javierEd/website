use leptos_spin::server_fn::register_explicit;
use leptos_spin::{render_best_match_to_stream, RouteTable};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::http_component;

#[http_component]
async fn handle_website(req: IncomingRequest, resp_out: ResponseOutparam) {
    let conf = leptos::get_configuration(None).await.unwrap();

    register_explicit::<crate::pages::GetPost>();
    register_explicit::<crate::server_functions::GetPosts>();

    let router = crate::main_router::MainRouter;

    let mut routes = RouteTable::build(router);
    routes.add_server_fn_prefix("/api").unwrap();

    render_best_match_to_stream(req, resp_out, &routes, router, &conf.leptos_options).await
}
