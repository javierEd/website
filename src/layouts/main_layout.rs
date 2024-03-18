use leptos::html::Span;
use leptos::*;
use leptos_bulma::components::{
    BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem, BNavbarMenu, BNavbarStart,
};
use leptos_meta::Title;
use leptos_use::use_interval_fn;
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
                let _ = element.set_attribute(
                    "style",
                    &format!("scroll-behavior: smooth; max-width: {}px", last_child_width),
                );
            }

            element.set_scroll_top(element.scroll_height());
        }
    });

    view! {
        <span class=format!("job-titles-carousel {}", class) node_ref=node_ref>
            <For each=move || job_titles.get() key=|pn| pn.to_owned() children=move |pn| { view! { <div>{pn}</div> } }/>
        </span>
    }
}

#[component]
pub fn MainLayout(children: Children, title: &'static str) -> impl IntoView {
    let job_titles = [
        "Software Developer",
        "Backend Developer",
        "Frontend Developer",
        "Fullstack Developer",
        "Ruby Developer",
        "Rust Developer",
        "Javascript Developer",
        "React Developer",
        "Mobile Developer",
        "Flutter Developer",
        "Software Engineer",
        "Backend Engineer",
        "Frontend Engineer",
        "Fullstack Engineer",
        "Ruby Engineer",
        "Rust Engineer",
        "Javascript Engineer",
        "React Engineer",
        "Mobile Engineer",
        "Flutter Engineer",
        "Programmer",
        "Coder",
    ];

    let burger_is_active = create_rw_signal(false);
    let (job_title, set_job_title) = create_signal(job_titles[0]);
    let (job_title_index, set_job_title_index) = create_signal(0);

    provide_context(JobTitle(job_title));

    use_interval_fn(
        move || {
            set_job_title_index.set((job_title_index.get() + 1) % job_titles.len());
            set_job_title.set(job_titles[job_title_index.get()]);
        },
        2500,
    );

    let title_view = move || {
        view! {<Title text=format!("{} | Javier E. - {}", title, job_title.get())/> }
    };

    view! {
        {title_view}

        <BNavbar class="is-black">
            <BNavbarBrand>
                <BNavbarItem class="media" href="/">
                    <div class="media-left">
                        <figure class="image is-48x48">
                            <img class="is-rounded" src="/images/favicon.png"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <div class="title is-5 has-text-light">Javier E.</div>
                        <div class="subtitle is-6 has-text-grey-lighter"><JobTitlesCarousel/></div>
                    </div>
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem href="/">Home</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <a class="navbar-item" href="https://github.com/javierEd" target="_blank" title="GitHub">
                        <span class="icon mx-1">
                            <img src="/images/github-icon-light.svg" alt="GitHub"/>
                        </span>
                    </a>
                    <a class="navbar-item" href="https://gitlab.com/javier.ed" target="_blank" title="GitHub">
                        <span class="icon mx-1">
                            <img src="/images/gitlab-icon-light.svg" alt="GitLab"/>
                        </span>
                    </a>
                    <a class="navbar-item" href="https://linkedin.com/in/javiered" target="_blank" title="LinkedIn">
                        <span class="icon mx-1">
                            <img src="/images/linkedin-icon-light.svg" alt="GitLab"/>
                        </span>
                    </a>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>

        <main class="container my-4">
            {children()}
        </main>

        <footer class="footer">
            <div class="content">
                <div class="is-flex is-align-items-center is-justify-content-center">
                    This website is powered by
                    <a class="mx-3" href="https://leptos.dev" target="_blank" title="Go to Leptos">
                        <img src="/images/leptos-logo.svg" alt="Leptos" width="100"/>
                    </a>
                    &
                    <a class="mx-3" href="https://bulma.io/" target="_blank" title="Go to Bulma">
                        <img src="/images/bulma-logo.svg" alt="Bulma" width="100"/>
                    </a>
                </div>
                <div class="mt-3 is-flex is-align-items-center is-justify-content-center">
                    And you can see the source code at
                    <a class="mx-3" href="https://github.com/javierEd/website" target="_blank" title="Go to GitHub">
                        <img src="/images/github-logo.svg" alt="GitHub" width="100"/>
                    </a>
                </div>
            </div>
        </footer>
    }
}
