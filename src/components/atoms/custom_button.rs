use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Option<Callback<()>>,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let onclick_internal = Callback::from(move |_| {
        if let Some(onclick) = &onclick {
            onclick.emit(());
        }
    });
    html! {
        <button onclick={ onclick_internal }>{ &props.label }</button>
    }
}
