use stylist::yew::styled_component;
use yew::prelude::*;

mod components;
mod router;

use crate::components::atoms::struct_hello::StructHello;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <StructHello />
        </div>
    }
}
