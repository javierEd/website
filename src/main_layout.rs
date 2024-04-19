use leptos::*;
use leptos_bulma::columns::{BColumn, BColumns};
use leptos_bulma::components::{
    BDropdown, BDropdownItem, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem, BNavbarMenu, BNavbarStart,
};
use leptos_bulma::elements::{BButton, BButtons, BIcon, BSubtitle, BTitle};
use leptos_bulma::icons::icondata_fa;
use leptos_use::{use_interval_fn, ColorMode};

use crate::components::{JobTitle, JobTitlesCarousel};
use crate::i18n::{t, use_i18n, Locale};
use crate::main_router::use_app_color_mode;

#[component]
fn ImageColorModes(dark_src: &'static str, light_src: &'static str, alt: &'static str, width: i8) -> impl IntoView {
    let color_mode = use_app_color_mode().mode;

    view! {
        <picture>
            <Show when=move || [ColorMode::Dark, ColorMode::Auto].contains(&color_mode.get())>
                <source srcset=dark_src media="(prefers-color-scheme: dark)"/>
            </Show>

            <Show when=move || [ColorMode::Light, ColorMode::Auto].contains(&color_mode.get())>
                <source srcset=light_src media="(prefers-color-scheme: light)"/>
            </Show>

            <img
                src=move || { if color_mode.get() == ColorMode::Dark { dark_src } else { light_src } }
                alt=alt
                width=width
            />
        </picture>
    }
}

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let i18n = use_i18n();
    let app_color_mode = use_app_color_mode();
    let color_mode = app_color_mode.mode.get();
    let color_mode_is_dark = create_rw_signal(color_mode == ColorMode::Dark);
    let color_mode_is_light = create_rw_signal(color_mode == ColorMode::Light);
    let color_mode_is_auto = create_rw_signal(color_mode == ColorMode::Auto);

    create_effect(move |_| {
        let color_mode = app_color_mode.mode.get();

        color_mode_is_dark.set(color_mode == ColorMode::Dark);
        color_mode_is_light.set(color_mode == ColorMode::Light);
        color_mode_is_auto.set(color_mode == ColorMode::Auto);
    });

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
        <BNavbar class="is-black has-shadow">
            <BNavbarBrand>
                <BNavbarItem class="media mb-0" href="/">
                    <div class="media-left">
                        <figure class="image is-48x48">
                            <img class="is-rounded" src="/images/favicon.png"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <BTitle is=5 class="has-text-light">
                            "Javier E."
                        </BTitle>
                        <BSubtitle is=6 class="has-text-grey-lighter">
                            <JobTitlesCarousel/>
                        </BSubtitle>
                    </div>
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem href="/">{t!(i18n, home)}</BNavbarItem>
                    <BNavbarItem href="/about">{t!(i18n, about)}</BNavbarItem>
                    <BNavbarItem href="/blog">{t!(i18n, blog)}</BNavbarItem>
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

        <main class="container">
            <div class="m-5">{children()}</div>
        </main>

        <footer class="footer">
            <div class="content container">
                <BColumns>
                    <BColumn>
                        <div class="is-flex is-align-items-center is-justify-content-center">
                            {t!(i18n, this_website_was_made_with)}
                            <a class="mx-3" href="https://leptos.dev" target="_blank" title="Go to Leptos">
                                <ImageColorModes
                                    dark_src="/images/leptos-logo-light.svg"
                                    light_src="/images/leptos-logo.svg"
                                    alt="Leptos"
                                    width=100
                                />
                            </a> & <a class="mx-3" href="https://bulma.io/" target="_blank" title="Go to Bulma">
                                <ImageColorModes
                                    dark_src="/images/bulma-logo-light.svg"
                                    light_src="/images/bulma-logo.svg"
                                    alt="Bulma"
                                    width=100
                                />
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
                                <ImageColorModes
                                    dark_src="/images/github-logo-light.svg"
                                    light_src="/images/github-logo.svg"
                                    alt="GitHub"
                                    width=100
                                />
                            </a>
                        </div>
                    </BColumn>

                    <BColumn is="narrow has-text-right">
                        <BDropdown
                            class="mb-4"
                            is_right=true
                            is_up=true
                            trigger=move || {
                                view! { <span class="has-text-weight-bold">{t!(i18n, change_language)} " ▲"</span> }
                            }
                        >

                            <BDropdownItem on:click=move |_| i18n.set_locale(Locale::en)>"English"</BDropdownItem>
                            <BDropdownItem on:click=move |_| i18n.set_locale(Locale::es)>"Español"</BDropdownItem>
                        </BDropdown>

                        <BButtons has_addons=true>
                            <BButton
                                class="ml-auto"
                                title="System theme"
                                is_active=color_mode_is_auto
                                on:click=move |_| app_color_mode.set_mode.set(ColorMode::Auto)
                            >
                                <BIcon is_scaled=false icon=icondata_fa::FaDesktopSolid/>
                            </BButton>
                            <BButton
                                title="Light theme"
                                is_active=color_mode_is_light
                                on:click=move |_| app_color_mode.set_mode.set(ColorMode::Light)
                            >
                                <BIcon is_scaled=false icon=icondata_fa::FaSunSolid/>
                            </BButton>
                            <BButton
                                title="Dark theme"
                                is_active=color_mode_is_dark
                                on:click=move |_| app_color_mode.set_mode.set(ColorMode::Dark)
                            >
                                <BIcon is_scaled=false icon=icondata_fa::FaMoonSolid/>
                            </BButton>
                        </BButtons>
                    </BColumn>
                </BColumns>
            </div>
        </footer>
    }
}
