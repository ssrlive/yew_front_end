use yew::{html, Component, Context, Html};

pub enum StructCounterMsg {
    ButtonClicked(i32),
}

pub struct StructCounter {
    pub count: i32,
}

impl Component for StructCounter {
    type Message = StructCounterMsg;
    type Properties = ();

    fn create(_props: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick = { context.link().callback(|_| StructCounterMsg::ButtonClicked(2)) } >{ "Increment" }</button>
                <p>{ "I have been clicked " }{ self.count }{" times"}</p>
            </div>
        }
    }

    fn update(&mut self, _context: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StructCounterMsg::ButtonClicked(amount) => {
                self.count += amount;
                true
            }
        }
    }
}
