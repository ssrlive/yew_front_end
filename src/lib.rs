use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

mod components;

use crate::components::{
    atoms::main_title::{Color, MainTitle},
    molecules::custom_form::{CustomForm, Data},
};

const STYLE_FILE: &str = include_str!("../main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let main_tilte_load = Callback::from(|msg| {
        log!(format!("MainTitle loaded: {msg}"));
    });

    let custom_form_onsubmit = Callback::from(|data: Data| {
        log!(format!("CustomForm submitted: {data:?}"));
    });

    html! {

        <div class={ stylesheet }>
            <MainTitle title="Some others" color={Color::Error} on_load={ main_tilte_load }/>
            <p>{ "This is a paragraph" }</p>
            <CustomForm onsubmit={ custom_form_onsubmit }/>
        </div>

    }
}
