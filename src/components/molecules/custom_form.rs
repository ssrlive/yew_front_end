use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};
use std::ops::Deref;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_change: Option<Callback<String>>,
}

#[derive(Clone, Default)]
struct Data {
    username: String,
    button_click_count: i32,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(Data::default);

    let cloned_state = state.clone();
    let on_change0 = props.on_change.clone();
    let on_change = Callback::from(move |value: String| {
        let mut data = cloned_state.deref().clone();
        data.username = value.clone();
        cloned_state.set(data);
        if let Some(on_change) = &on_change0 {
            on_change.emit(value);
        }
    });

    let cloned_state = state.clone();
    let button_on_click = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.button_click_count += 1;
        cloned_state.set(data);
    });

    html! {
        <div>
            <TextInput name="username" on_change={ on_change }/>
            <CustomButton label="Submit" onclick={ button_on_click }/>
            <p>{ "Username: " }{ &state.username }</p>
            <p>{ "Button clicked: " }{ state.button_click_count }{ " times" }</p>
        </div>
    }
}
