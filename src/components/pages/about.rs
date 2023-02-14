use crate::router::MyRoute;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};

#[function_component(About)]
pub fn about() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick_cb = Callback::from(move |_| {
        navigator.push(&MyRoute::Home);
    });
    html! {
        <div>
            <h1>{ "About" }</h1>
            <div>
                <button onclick = { onclick_cb } >{ "Go home" }</button>
            </div>
            <br />
            <div>
                <Link<MyRoute> to={ MyRoute::Home } >{ "Home" }</Link<MyRoute>>
            </div>
        </div>
    }
}
