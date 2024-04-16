use leptos::html::Input;
use leptos::*;
use leptos_bulma::elements::{BBox, BNotification};
use leptos_bulma::form::{BCheckboxField, BTextField, BTextareaField};

use super::SubmitButton;

#[component]
pub fn PostForm(
    action_value: RwSignal<Option<Result<bool, ServerFnError>>>,
    is_pending: ReadSignal<bool>,
    #[prop(optional)] id: String,
    #[prop(optional)] title: String,
    #[prop(optional)] slug: String,
    #[prop(optional)] content: String,
    #[prop(default = false)] publish: bool,
) -> impl IntoView {
    let show_error_msg = create_rw_signal(false);
    let slug_value = create_rw_signal(slug);
    let publish_node_ref = create_node_ref::<Input>();

    create_effect(move |_| {
        show_error_msg.set(action_value.get() == Some(Ok(false)));
    });

    create_effect(move |_| {
        if let Some(element) = publish_node_ref.get() {
            element.set_checked(publish);
        }
    });

    let title_on_input = move |event| {
        slug_value.set(slug::slugify(event_target_value(&event)));
    };

    view! {
        <BBox>
            <BNotification class="is-danger is-light" is_active=show_error_msg>
                "Failed to save post"
            </BNotification>

            <input type="hidden" name="id" value=id/>

            <BTextField id="title" label="Title" name="title" value=title on_input=title_on_input/>

            <BTextField id="slug" label="Slug" name="slug" value=slug_value/>

            <BTextareaField id="content" label="Content" name="content" value=content/>

            <BCheckboxField node_ref=publish_node_ref label="Publish" name="publish"/>

            <SubmitButton is_loading=is_pending.get()/>
        </BBox>
    }
}
