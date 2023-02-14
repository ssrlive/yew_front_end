use crate::components::atoms::main_title::MainTitle;
use stylist::{style, Style};
use yew::{html, Component, Context, Html};

pub struct StructHello {
    pub message: String,
    pub stylesheet: Style,
}

impl StructHello {
    fn style() -> Style {
        style!(
            r#"
            color: green;
            "#
        )
        .unwrap()
    }
}

impl Component for StructHello {
    type Message = ();
    type Properties = ();

    fn create(_props: &Context<Self>) -> Self {
        Self {
            message: "Hello, world from a struct".to_string(),
            stylesheet: Self::style(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1 class={ self.stylesheet.clone() }>{ &self.message }</h1>
                <MainTitle title = { self.message.clone() } />
            </div>
        }
    }
}
