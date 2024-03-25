use leptos::*;
use leptos_bulma::columns::{BColumn, BColumns};
use leptos_bulma::components::{
    BDropdown, BDropdownItem, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem, BNavbarMenu, BNavbarStart,
};
use leptos_use::use_interval_fn;

use crate::components::{JobTitle, JobTitlesCarousel};
use crate::i18n::{t, use_i18n, Locale};

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let i18n = use_i18n();

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
        "Loving Husband",
        "Amateur Gardener",
        "Aikidoka",
        "Iaijutsuka",
        "Human Being",
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

    view! {
        <BNavbar class="is-black">
            <BNavbarBrand>
                <BNavbarItem class="media" href="/">
                    <div class="media-left">
                        <figure class="image is-48x48">
                            <img class="is-rounded" src="/images/favicon.png"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <div class="title is-5 has-text-light">"Javier E."</div>
                        <div class="subtitle is-6 has-text-grey-lighter"><JobTitlesCarousel/></div>
                    </div>
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem href="/">{t!(i18n, home)}</BNavbarItem>
                    <BNavbarItem href="/about">{t!(i18n, about)}</BNavbarItem>
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

        <main class="container"><div class="m-5">{children()}</div></main>

        <footer class="footer">
            <div class="content container">
                <BColumns>
                    <BColumn>
                        <div class="is-flex is-align-items-center is-justify-content-center">
                            {t!(i18n, this_website_was_made_with)}
                            <a class="mx-3" href="https://leptos.dev" target="_blank" title="Go to Leptos">
                                <img src="/images/leptos-logo.svg" alt="Leptos" width="100"/>
                            </a>
                            &
                            <a class="mx-3" href="https://bulma.io/" target="_blank" title="Go to Bulma">
                                <img src="/images/bulma-logo.svg" alt="Bulma" width="100"/>
                            </a>
                        </div>
                        <div class="mt-3 is-flex is-align-items-center is-justify-content-center">
                            {t!(i18n, and_you_can_see_the_source_code_at)}
                            <a
                                class="mx-3"
                                href="https://github.com/javierEd/website"
                                target="_blank"
                                title="Go to GitHub"
                            >
                                <img src="/images/github-logo.svg" alt="GitHub" width="100"/>
                            </a>
                        </div>
                    </BColumn>

                    <BColumn is="narrow">
                        <BDropdown
                            trigger=|| view! {
                                <span class="has-text-weight-bold">{t!(i18n, change_language)}" ▼"</span>
                            }
                        >
                            <BDropdownItem on_click=move|_| i18n.set_locale(Locale::en)>"English"</BDropdownItem>
                            <BDropdownItem on_click=move|_| i18n.set_locale(Locale::es)>"Español"</BDropdownItem>
                        </BDropdown>
                    </BColumn>
                </BColumns>
            </div>
        </footer>
    }
}
