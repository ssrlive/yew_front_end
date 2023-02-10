use crate::components::atoms::{custom_button::CustomButton, text_input::TextInput};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_change: Option<Callback<String>>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    html! {
        <form>
            <TextInput name="username" on_change={ props.on_change.clone() }/>
            <CustomButton label="Submit"/>
        </form>
    }
}
