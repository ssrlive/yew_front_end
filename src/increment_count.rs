use crate::stores::counter_store::CounterStore;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

pub enum Msg {
    Store(Rc<CounterStore>),
    ButtonClicked,
}

pub struct IncrementCount {
    pub dispatch: Dispatch<CounterStore>,
}

impl Component for IncrementCount {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<CounterStore>::subscribe(ctx.link().callback(Msg::Store));
        Self { dispatch }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Store(_) => false,
            Msg::ButtonClicked => {
                self.dispatch.reduce(|store| {
                    CounterStore {
                        count: store.count + 1,
                    }
                    .into()
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::ButtonClicked);
        html! {
            <button {onclick} > { "Increment Count" } </button>
        }
    }
}
