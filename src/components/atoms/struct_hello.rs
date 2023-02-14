use crate::components::atoms::main_title::MainTitle;
use stylist::{style, Style};
use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub message: String,
}

pub struct StructHello {
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
    type Properties = Props;

    fn create(_props: &Context<Self>) -> Self {
        Self {
            stylesheet: Self::style(),
        }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            <div>
                <h1 class={ self.stylesheet.clone() }>{ &context.props().message }</h1>
                <MainTitle title = { context.props().message.clone() } />
            </div>
        }
    }
}
