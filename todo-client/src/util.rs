use yew::{platform::{pinned::oneshot, spawn_local}, prelude::*};
use web_sys::{self, HtmlInputElement, wasm_bindgen::JsCast};

pub fn textinput_value_from_event(e:Event) -> String {
    let target: HtmlInputElement = e.target().unwrap().unchecked_into();
    target.value()
}

// this will cause problems if it isn't used on an input type="text"
pub fn textinput_callback(handle: UseStateHandle<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        handle.set(textinput_value_from_event(e));
    })
}
