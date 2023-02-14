use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Home" }</h1>
        </div>
    }
}
