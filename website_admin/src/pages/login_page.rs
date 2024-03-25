use leptos::*;
use leptos_bulma::columns::BColumn;
use leptos_bulma::elements::{BBox, BNotification};
use leptos_bulma::form::{BPasswordField, BTextField};
use leptos_router::ActionForm;

use crate::components::{PageTitle, RequireNoAuthentication, SubmitButton};

#[server(prefix = "/admin/api")]
pub async fn attempt_to_login(username: String, password: String) -> Result<bool, ServerFnError> {
    crate::server_functions::require_no_authentication().await?;

    use crate::variables::{admin_password, admin_username};

    let is_valid = username == admin_username() && password == admin_password();

    if is_valid {
        crate::server_functions::start_admin_session()?;

        leptos_spin::redirect("/admin");
    }

    Ok(is_valid)
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let server_action = create_server_action::<AttemptToLogin>();
    let action_value = server_action.value();
    let show_error_msg = create_rw_signal(false);

    create_effect(move |_| {
        show_error_msg.set(action_value.get() == Some(Ok(false)));
    });

    view! {
        <RequireNoAuthentication/>

        <PageTitle text="Login"/>

        <main class="container">
            <div class="m-5">
                <BColumn is="6" is_offset="3">
                    <h2 class="title is-1 has-text-centered">"Login"</h2>

                    <BBox>
                        <BNotification class="is-danger is-light" is_active=show_error_msg>
                            "Failed to authenticate admin"
                        </BNotification>

                        <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true">
                            <BTextField id="username" label="Username" name="username"/>

                            <BPasswordField id="password" label="Password" name="password"/>

                            <SubmitButton is_loading=server_action.pending()/>
                        </ActionForm>
                    </BBox>
                </BColumn>
            </div>
        </main>
    }
}
