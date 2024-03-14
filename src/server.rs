use leptos_spin::{render_best_match_to_stream, RouteTable};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::http_component;

#[http_component]
async fn handle_website(req: IncomingRequest, resp_out: ResponseOutparam) {
    let conf = leptos::get_configuration(None).await.unwrap();

    let app_router = crate::routes::AppRouter;

    let mut routes = RouteTable::build(app_router);
    routes.add_server_fn_prefix("/api").unwrap();

    render_best_match_to_stream(req, resp_out, &routes, app_router, &conf.leptos_options).await
}
