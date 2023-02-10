use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

mod components;

use crate::components::{
    atoms::main_title::{Color, MainTitle},
    molecules::custom_form::CustomForm,
};

const STYLE_FILE: &str = include_str!("../main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let main_tilte_load = Callback::from(|msg| {
        log!(format!("MainTitle loaded: {msg}"));
    });

    let input_change = Callback::from(|value: String| {
        log!(value);
    });

    html! {

        <div class={ stylesheet }>
            <MainTitle title="Some others" color={Color::Error} on_load={ main_tilte_load }/>
            <p>{ "This is a paragraph" }</p>
            <CustomForm on_change={ input_change }/>
        </div>

    }
}
