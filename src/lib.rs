use gloo::console::log;
use serde::{Deserialize, Serialize};
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
    log!(serde_json::to_string_pretty(&obj).unwrap());

    let class = "my_title";

    let tasks = vec!["task1", "task2", "task3"];

    html! {
        <>

        <h1 class= { class } >{ format!("Hello, {}!", name) }</h1>
        <p>{ "This is a paragraph" }</p>

        <ul>
            { for tasks.iter().map(|task| html! { <li>{ task }</li> }) }
            // or
            // { tasks.iter().map(|task| html! { <li>{ task }</li> }).collect::<Html>() }
        </ul>

        </>
    }
}
