use crate::components::atoms::main_title::MainTitle;
use yew::{html, Component, Context, Html};

pub struct StructHello {
    pub message: String,
}

impl Component for StructHello {
    type Message = ();
    type Properties = ();

    fn create(_props: &Context<Self>) -> Self {
        Self {
            message: "Hello, world from a struct".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                // <h1>{ &self.message }</h1>
                <MainTitle title = { self.message.clone() } />
            </div>
        }
    }
}
