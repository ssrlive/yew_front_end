use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};
use gloo::console::log;
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

    let button_click_count = use_state(|| 0);
    let state = button_click_count.clone();
    let button_on_click = Callback::from(move |_| {
        let count = *state;
        state.set( count + 1);
    });

    html! {
        <div>
            <TextInput name="username" on_change={ on_change }/>
            <CustomButton label="Submit" onclick={ button_on_click }/>
            <p>{ "Username: " }{ &*username_state }</p>
            <p>{ "Button clicked: " }{ *button_click_count }{ " times" }</p>
        </div>
    }
}
