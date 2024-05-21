use crate::common::prelude::*;
use enum_iterator::all;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LanguagePickerProps {
    pub selected: Language,
}

#[function_component]
pub fn LanguagePicker(props: &LanguagePickerProps) -> Html {
    let (_, dispatch) = use_store::<CalcState>();
    let onchange = dispatch.reduce_callback_with(|state, event: Event| {
        let target: EventTarget = event.target().unwrap();
        let value = target.dyn_into::<HtmlSelectElement>().unwrap().value();
        let lang = value.try_into().unwrap();
        CalcState {
            language: lang,
            ..(*state).clone()
        }
        .into()
    });
    let langs = all::<Language>()
        .map(|l| {
            html! {
                <option value={String::from(&l)} selected={props.selected == l}>{l.as_caption()}</option>
            }
        })
        .collect::<Vec<_>>();
    html! {
        <select class="appearance-none rounded-none bg-white border-gray-300 p-1 border-solid border dark:bg-gray-700 dark:text-gray-300" {onchange} title={"Pick language"}>
        {langs}
        </select>
    }
}
