use leptos::*;
use leptos_bulma::components::{
    BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem, BNavbarMenu, BNavbarStart,
};
use leptos_meta::Title;

#[component]
pub fn MainLayout(children: Children, title: &'static str) -> impl IntoView {
    let burger_is_active = create_rw_signal(false);

    view! {
        <Title text=format!("{} | Javier E.", title)/>

        <BNavbar class="is-black">
            <BNavbarBrand>
                <BNavbarItem class="media" href="/">
                    <div class="media-left">
                        <figure class="image is-48x48">
                            <img class="is-rounded" src="/images/favicon.png"/>
                        </figure>
                    </div>
                    <div class="media-content has-text-centered">
                        <div class="title is-5 has-text-light">Javier E.</div>
                        <div class="subtitle is-6 has-text-grey-lighter">Software developer</div>
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
