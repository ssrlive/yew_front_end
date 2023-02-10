use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: Option<String>,
    pub color: Option<Color>,
    pub on_load: Option<Callback<String>>,
}

#[allow(dead_code)]
#[derive(Default, PartialEq)]
pub enum Color {
    #[default]
    Normal,
    Ok,
    Error,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Normal => write!(f, "normal"),
            Color::Ok => write!(f, "ok"),
            Color::Error => write!(f, "error"),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let title = match &props.title {
        Some(title) => title,
        None => "No title",
    };

    let color = match &props.color {
        Some(color) => color,
        None => &Color::Normal,
    };

    let style = style!(
        r#"
        .normal {
            color: white;
        }
        .ok {
            color: green;
        }
        .error {
            color: red;
        }
    "#
    )
    .unwrap();

    if let Some(on_load) = &props.on_load {
        on_load.emit(title.to_string());
    }

    html! {
        <div class={ style }>
            <h1 class = { color.to_string() }>{ title }</h1>
        </div>
    }
}
