use yew::prelude::*;
use stylist::{yew::styled_component, style, Style};

const STYLE_FILE: &str = include_str!("../main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {

        <div class={ stylesheet }>
            <h1>{ "Hello World!" }</h1>
            <p class={ css!("color: green; font-size: 75px;") }>{ "This is a paragraph" }</p>
        </div>

    }
}
