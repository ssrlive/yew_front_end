use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_change: Option<Callback<String>>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let username_state = use_state(String::new);
    let state = username_state.clone();
    let on_change0 = props.on_change.clone();
    let on_change = Callback::from(move |value: String| {
        state.set(value.clone());
        if let Some(on_change) = &on_change0 {
            on_change.emit(value);
        }
    });

    html! {
        <form>
            <TextInput name="username" on_change={ on_change }/>
            <CustomButton label="Submit"/>
            <p>{ "Username: " }{ &*username_state }</p>
        </form>
    }
}
