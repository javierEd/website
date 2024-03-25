use leptos::html::Span;
use leptos::*;
use leptos_meta::Title;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

#[derive(Clone)]
pub struct JobTitle(pub ReadSignal<&'static str>);

#[component]
pub fn JobTitlesCarousel(#[prop(default = "")] class: &'static str) -> impl IntoView {
    let job_title = use_context::<JobTitle>().unwrap();
    let job_titles = create_rw_signal(vec![job_title.0.get()]);

    let node_ref = create_node_ref::<Span>();

    create_effect(move |_| {
        let mut new_job_titles = job_titles.get();

        new_job_titles.push(job_title.0.get());

        if new_job_titles.len() > 2 {
            let _ = new_job_titles.remove(0);
        }

        job_titles.set(new_job_titles);

        if let Some(element) = node_ref.get() {
            let _ = element.remove_attribute("style");
            element.set_scroll_top(0);

            if let Some(last_child) = element.child_nodes().get(1).unwrap().dyn_ref::<HtmlDivElement>() {
                let last_child_width = last_child.client_width();
                let last_child_height = last_child.client_height();
                let _ = element.set_attribute(
                    "style",
                    &format!(
                        "scroll-behavior: smooth; max-width: {}px; max-height: {}px",
                        last_child_width, last_child_height
                    ),
                );
            }

            element.set_scroll_top(element.scroll_height());
        }
    });

    view! {
        <span class=format!("job-titles-carousel {}", class) node_ref=node_ref>
            <For each=move || job_titles.get() key=|jt| jt.to_owned() children=move |jt| view! { <div>{jt}</div> }/>
        </span>
    }
}

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    let job_title = use_context::<JobTitle>().unwrap();

    move || view! { <Title text=format!("{} | Javier E. - {}", text.get(), job_title.0.get())/> }
}
