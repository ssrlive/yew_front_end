use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello, world!" }</h1>
        </div>
    }
}
