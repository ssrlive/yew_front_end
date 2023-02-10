use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub name: String,
    pub on_change: Option<Callback<String>>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let on_change = props.on_change.clone();
    let on_change_internal = Callback::from(move |event: Event| {
        if let Some(on_change) = &on_change {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            on_change.emit(value);
        }
    });

    html! {
        <input type="text" name={ props.name.clone()} onchange={ on_change_internal }/>
    }
}
