use crate::router::MyRoute;
use yew::{function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <div>
                <Link<MyRoute> to={ MyRoute::About } >{ "About" }</Link<MyRoute>>
            </div>
        </div>
    }
}
