use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("../main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {

        <div class={ stylesheet }>
            <h1>{ "Hello World!" }</h1>
            <p>{ "This is a paragraph" }</p>
        </div>

    }
}
