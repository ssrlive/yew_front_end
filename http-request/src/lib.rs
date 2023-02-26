use gloo::console::log;
use reqwasm::http::Request;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct User {
    pub token: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let state = use_state(|| {
        User{
        token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImJvbzZvIiwiaWF0IjoxNjc3MzgwNjM3fQ.B0a3DkIy8xP6ax0cTIZMuOUygfqpd61UrOYq7NZDBmY".to_string(),
    }
    });

    let on_click = {
        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("")
                    .header("x-auth-token", &state.token)
                    .send()
                    .await
                    .unwrap();
                log!(response.status());
            });
        })
    };

    html! {
        <div>
            <h1>{ "Hello, world!" }</h1>
            <button onclick={ on_click }>{ "Click me!" }</button>
        </div>
    }
}
