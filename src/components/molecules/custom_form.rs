use crate::{
    components::atoms::{custom_button::CustomButton, text_input::TextInput},
    User,
};
use std::ops::Deref;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Option<Callback<Data>>,
}

#[derive(Clone, Default, Debug)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(Data::default);

    let context = use_context::<User>();

    let cloned_state = state.clone();
    let on_change = Callback::from(move |value: String| {
        cloned_state.set(Data {
            username: value,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let language_changed = Callback::from(move |value: String| {
        cloned_state.set(Data {
            favorite_language: value,
            ..cloned_state.deref().clone()
        });
    });

    let form_onsubmit = props.onsubmit.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = state.deref().clone();
        if let Some(on_submit) = &form_onsubmit {
            on_submit.emit(data);
        }
    });

    html! {
        <form onsubmit={ onsubmit }>
            <TextInput name="username" on_change={ on_change }/>
            <TextInput name="favorite_language" on_change={ language_changed }/>
            <CustomButton label="Submit"/>
            <p>{ format!("Context: {:#?}", context.unwrap_or_default()) }</p>
        </form>
    }
}
