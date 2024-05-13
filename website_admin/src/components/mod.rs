use leptos::*;
use leptos_bulma::elements::BButton;
use leptos_bulma::enums::{BColor, BState};
use leptos_bulma::form::{BControl, BField};
use leptos_meta::Title;
use leptos_router::Redirect;

use crate::server_functions::{is_admin_enabled, is_authenticated};

mod post_form;

pub use post_form::*;

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    move || view! { <Title text=format!("{} | Javier E. - Admin Panel", text.get())/> }
}

#[component]
pub fn RequireAuthentication() -> impl IntoView {
    view! {
        <Await future=is_authenticated let:data>
            {if let Ok(false) = data { Some(view! { <Redirect path="/"/> }) } else { None }}
        </Await>
    }
}

#[component]
pub fn RequireNoAuthentication() -> impl IntoView {
    view! {
        <Await future=is_admin_enabled let:data>
            {if let Ok(false) = data {
                Some(view! { <Redirect path="/"/> })
            } else {
                Some(
                    view! {
                        <Await future=is_authenticated let:data>
                            {if let Ok(true) = data { Some(view! { <Redirect path="/admin"/> }) } else { None }}
                        </Await>
                    },
                )
            }}

        </Await>
    }
}

#[component]
pub fn SubmitButton(
    #[prop(default = "Submit")] label: &'static str,
    #[prop(optional, into)] is_loading: MaybeSignal<bool>,
) -> impl IntoView {
    let assign_button_state = move || {
        if is_loading.get() {
            BState::Active
        } else {
            BState::Default
        }
    };

    let button_state = create_rw_signal(assign_button_state());

    create_effect(move |_| {
        button_state.set(assign_button_state());
    });

    view! {
        <BField>
            <BControl>
                <BButton button_type="submit" color=BColor::Info is_fullwidth=true state=button_state>
                    {label}
                </BButton>
            </BControl>
        </BField>
    }
}
