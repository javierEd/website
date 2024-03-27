use leptos::*;
use leptos_bulma::components::{
    BModal, BModalClose, BModalContent, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem, BNavbarMenu,
    BNavbarStart,
};
use leptos_bulma::elements::BBox;

use crate::components::{PageTitle, RequireAuthentication};

#[server(prefix = "/admin/api")]
pub async fn attempt_to_logout() -> Result<(), ServerFnError> {
    crate::server_functions::require_authentication().await?;

    crate::server_functions::delete_admin_session()?;

    leptos_spin::redirect("/admin/login");

    Ok(())
}

#[component]
pub fn AdminLayout(children: Children, #[prop(into)] title: TextProp) -> impl IntoView {
    let burger_is_active = create_rw_signal(false);
    let show_logout_confirmation = create_rw_signal(false);
    let action_logout = create_action(|_| attempt_to_logout());

    view! {
        <RequireAuthentication/>

        <PageTitle text=title/>

        <BNavbar class="is-dark">
            <BNavbarBrand>
                <BNavbarItem class="media mb-0" href="/admin">
                    <div class="media-left">
                        <figure class="image is-48x48">
                            <img class="is-rounded" src="/images/favicon.png"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <div class="title is-5 has-text-light">"Javier E."</div>
                        <div class="subtitle is-6 has-text-grey-lighter">"Admin Panel"</div>
                    </div>
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem href="/admin">"Home"</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <BNavbarItem href="/" target="_blank">"Go to website"</BNavbarItem>
                    <BNavbarItem on_click=move |_| show_logout_confirmation.set(true)>"Logout"</BNavbarItem>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>

        <BModal is_active=show_logout_confirmation>
            <BModalContent>
                <BBox class="has-text-centered">
                    <span class="icon is-large">
                        <span class="material-symbols-rounded">question_mark</span>
                    </span>
                    <h3 class="title is-4">"Are you sure you want to logout?"</h3>
                    <div class="buttons is-centered">
                        <a
                            class="button is-primary"
                            on:click=move |_| action_logout.dispatch(())
                        >
                            "Accept"
                        </a>
                        <a class="button" on:click=move |_| show_logout_confirmation.set(false)>
                            "Cancel"
                        </a>
                    </div>
                </BBox>
            </BModalContent>
            <BModalClose on_click=move |_| show_logout_confirmation.set(false)/>
        </BModal>

        <main class="container">
            <div class="m-5">{children()}</div>
        </main>
    }
}
