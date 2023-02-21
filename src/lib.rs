use stylist::yew::styled_component;
use yew::prelude::*;

mod components;
mod display_count;
mod increment_count;
mod router;
mod stores;

use crate::components::{
    atoms::struct_hello::StructHello, molecules::struct_counter::StructCounter,
};
use display_count::DisplayCount;
use increment_count::IncrementCount;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <StructHello message={ "Hello from lib.rs".to_string() } />
            <StructCounter />
            <DisplayCount />
            <IncrementCount />
        </div>
    }
}
