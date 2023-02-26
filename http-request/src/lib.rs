use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct User {
    pub token: String,
    pub tasks: Option<Vec<Task>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed_at: Option<String>,
    pub priority: Option<char>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResponse {
    pub data: Vec<Task>,
}

#[styled_component(App)]
pub fn app() -> Html {
    let state = use_state(|| {
        User{
        token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImJvbzZvIiwiaWF0IjoxNjc3MzgwNjM3fQ.B0a3DkIy8xP6ax0cTIZMuOUygfqpd61UrOYq7NZDBmY".to_string(),
        tasks: None,
    }
    });

    let on_click = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:3000/api/v1/tasks")
                    .header("x-auth-token", &state.token)
                    .send()
                    .await
                    .unwrap()
                    .json::<TaskResponse>()
                    .await
                    .unwrap();
                let mut user = state.deref().clone();
                user.tasks = Some(response.data);
                state.set(user);
            });
        })
    };

    html! {
        <div>
            <h1>{ "Hello, world!" }</h1>
            <button onclick={ on_click }>{ "Click me!" }</button>
            <pre>{ serde_json::to_string_pretty(&state.tasks).unwrap() }</pre>
        </div>
    }
}
