use crate::router::MyRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h1>{ "About" }</h1>
            <div>
                <Link<MyRoute> to={ MyRoute::Home } >{ "Home" }</Link<MyRoute>>
            </div>
        </div>
    }
}
