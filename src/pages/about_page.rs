use chrono::{NaiveDate, Utc};
use leptos::*;
use leptos_bulma::columns::{BColumn, BColumns};

use crate::layouts::MainLayout;

#[component]
pub fn AboutPage() -> impl IntoView {
    let start_date = NaiveDate::from_ymd_opt(2010, 10, 1).unwrap();

    let experience_years = move || Utc::now().date_naive().years_since(start_date).unwrap_or_default();

    let cumana_link_view = move || {
        view! {
            <a href="https://maps.app.goo.gl/TYt8nVUcte1JNmQ96" target="_blank">"Cumaná, Venezuela"</a>
        }
    };

    let projects = [(
        1,
        view! { <a href="https://github.com/javierEd/leptos-bulma" target="_blank">"Leptos Bulma"</a> },
        view! { "A Leptos component library based on Bulma CSS framework." },
    )];

    let experience = [
        (
            7,
            view! { <a href="https://www.comunidadfeliz.cl/" target="_blank">"ComunidadFeliz SpA"</a> }.into_view(),
            "Fullstack Developer",
            "Fulltime",
            view! { "Remote" }.into_view(),
            "May 2022 - January 2023",
        ),
        (
            6,
            view! { <a href="https://vascarsolutions.com/" target="_blank">"Vascar Solutions "</a> }.into_view(),
            "Backend Developer",
            "Freelance",
            view! { "Remote" }.into_view(),
            "April 2021 – December 2021",
        ),
        (
            5,
            view! { <a href="https://simgulary.io/" target="_blank">"SIM C.A."</a> }.into_view(),
            "Backend Developer",
            "Freelance",
            cumana_link_view.into_view(),
            "November 2020 – April 2021",
        ),
        (
            4,
            view! { "Seai Lab" }.into_view(),
            "Mobile Developer",
            "Freelance",
            view! { "Remote" }.into_view(),
            "October 2019 – April 2020",
        ),
        (
            3,
            view! { <a href="https://cesticom.com/" target="_blank">"CESTICOM C.A."</a> }.into_view(),
            "Software Developer",
            "Fulltime",
            cumana_link_view.into_view(),
            "May 2018 – June 2019",
        ),
        (
            2,
            view! { "Servicios VECA C.A." }.into_view(),
            "Backend Developer",
            "Fulltime",
            cumana_link_view.into_view(),
            "May 2017 – May 2018",
        ),
        (
            1,
            view! { "Sucre Municipality Town Hall" }.into_view(),
            "Technical Support/Software Developer",
            "Fulltime",
            cumana_link_view.into_view(),
            "October 2010 – April 2017",
        ),
    ];

    view! {
        <MainLayout title="About me">
            <h2 class="title is-1">"About me"</h2>

            <h3 class="subtitle is-3">
                <ul>
                    <li>"I'm a Software Developer with "{experience_years}" years of experience."</li>
                    <li>
                        "I've been in "
                        <a href="https://maps.app.goo.gl/T3HN2x8KL44DhxDB9" target="_blank">
                            "Buenos Aires, Argentina"
                        </a>
                        " since October 2022."
                    </li>
                    <li>"But I was born and raised in "{cumana_link_view}"."</li>
                </ul>
            </h3>

            <section class="section">
                <h4 class="title mb-1">"Open Source Projects"</h4>

                <For each=move|| projects.clone() key=|p| p.0 children=move |p| view! {
                    <div class="pt-5">
                        <h5 class="title is-4">{p.1}</h5>
                        <h6 class="subtitle">{p.2}</h6>
                    </div>
                }/>
            </section>

            <section class="section">
                <h4 class="title mb-1">"Professional Experience"</h4>

                <For each=move|| experience.clone() key=|exp| exp.0 children=move |exp| view! {
                    <div class="pt-5">
                        <h5 class="title is-4">{exp.1}</h5>
                        <h6 class="subtitle">
                            <BColumns>
                                <BColumn>{exp.2}</BColumn>
                                <BColumn>{exp.3}</BColumn>
                                <BColumn>{exp.4}</BColumn>
                                <BColumn>{exp.5}</BColumn>
                            </BColumns>
                        </h6>
                    </div>
                }/>
            </section>
        </MainLayout>
    }
}
