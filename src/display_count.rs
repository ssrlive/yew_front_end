use crate::stores::counter_store::CounterStore;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

pub struct DisplayCount {
    dispatch: Dispatch<CounterStore>,
}

pub enum Msg {
    Store(Rc<CounterStore>),
}

impl Component for DisplayCount {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<CounterStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    // fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    //     match msg {
    //         Msg::Store(_) => true,
    //     }
    // }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let count = self.dispatch.get().count;
        html! {
            <div>
                <h2>{ "Display Count" }</h2>
                <div>{ count }</div>
            </div>
        }
    }
}
