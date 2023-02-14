use gloo::console::log;
use std::ops::Deref;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod router;

use crate::{
    components::{
        atoms::{
            main_title::{Color, MainTitle},
            struct_hello::StructHello,
        },
        molecules::custom_form::{CustomForm, Data},
    },
    router::{my_switch, MyRoute},
};

const STYLE_FILE: &str = include_str!("../main.css");

#[derive(Clone, Debug, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let user_state = use_state(User::default);

    let main_tilte_load = Callback::from(|msg: String| {
        log!(format!("MainTitle loaded: {msg}"));
    });

    let cloned_user_state = user_state.clone();
    let custom_form_onsubmit = Callback::from(move |data: Data| {
        cloned_user_state.set(User {
            username: data.username,
            favorite_language: data.favorite_language,
        });
    });

    let first_load = use_state(|| {
        log!("App loaded");
        true
    });
    use_effect(move || {
        if *first_load.deref() {
            first_load.set(false);
        } else {
            log!("App updated");
        }
        || log!("App unmounted")
    });

    html! {
        <div class={ stylesheet }>
        <ContextProvider<User> context={ user_state.deref().clone() }>
            <MainTitle title="Some others" color={Color::Error} on_load={ main_tilte_load }/>
            <p>{ "This is a paragraph" }</p>
            <CustomForm onsubmit={ custom_form_onsubmit }/>
            <BrowserRouter>
                <Switch<MyRoute> render={ my_switch } />
            </BrowserRouter>
            <StructHello />
        </ContextProvider<User>>
        </div>
        // <div>
        //     <StructHello />
        // </div>
    }
}
