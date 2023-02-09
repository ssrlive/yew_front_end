use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use stylist::{yew::styled_component, style};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MyOjbect {
    name: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = style!(
        r#"
            h1 {
                color: red;
            }
        "#
    ).unwrap();
    html! {

        <div class={ stylesheet }>
            <h1>{ "Hello World!" }</h1>
            <p class={ css!("color: green; font-size: 75px;") }>{ "This is a paragraph" }</p>
        </div>

    }
}
