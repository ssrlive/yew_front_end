use crate::components::pages::{about::About, home::Home};
use yew::{html, Html};
use yew_router::Routable;

#[derive(PartialEq, Debug, Clone, Routable)]
pub enum MyRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/404")]
    NotFound,
}

#[allow(dead_code)]
pub fn my_switch(routes: MyRoute) -> Html {
    match routes {
        MyRoute::Home => html! { <Home /> },
        MyRoute::About => html! { <About /> },
        MyRoute::Contact => html! { <h1>{ "Contact" }</h1> },
        MyRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
