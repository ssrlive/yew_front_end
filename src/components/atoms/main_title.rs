use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: Option<String>,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let title = match &props.title {
        Some(title) => title,
        None => "No title",
    };
    html! {
        <h1>{ title }</h1>
    }
}
