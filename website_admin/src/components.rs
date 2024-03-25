use leptos::*;
use leptos_bulma::elements::BButton;
use leptos_bulma::form::{BControl, BField};
use leptos_meta::Title;
use leptos_router::Redirect;

use crate::server_functions::{is_admin_enabled, is_authenticated};

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
            {if let Ok(false) = data { Some(view! { <Redirect path="/"/> }) } else { Some(view! {
                <Await future=is_authenticated let:data>
                    {if let Ok(true) = data { Some(view! { <Redirect path="/admin"/> }) } else { None }}
                </Await>
            }) }}
        </Await>
    }
}

#[component]
pub fn SubmitButton(
    #[prop(default = "Submit")] label: &'static str,
    #[prop(optional, into)] is_loading: MaybeSignal<bool>,
) -> impl IntoView {
    view! {
        <BField>
            <BControl>
                <BButton button_type="submit" class="button is-info is-fullwidth" is_loading=is_loading>{label}</BButton>
            </BControl>
        </BField>
    }
}
