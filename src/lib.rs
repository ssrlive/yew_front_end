use serde::{Deserialize, Serialize};
use gloo::console::log;
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MyOjbect {
    name: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "World";

    let obj = MyOjbect {
        name: name.to_string(),
    };

    log!(format!("Hello, {name}!"));
    log!( serde_json::to_string_pretty(&obj).unwrap());

    html! {
        <h1>{ format!("Hello, {}!", name) }</h1>
    }
}
