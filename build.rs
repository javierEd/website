#[tokio::main]
async fn main() {
    let conf = leptos::get_configuration(None).await.unwrap();

    let output_path = std::path::Path::new(&conf.leptos_options.site_root).join(&conf.leptos_options.site_pkg_dir);

    println!("cargo:rerun-if-changed={}", output_path.to_str().unwrap_or_default());

    leptos_bulma::LeptosBulma::setup(&conf.leptos_options);
}
