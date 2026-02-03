use crate::{
    common::prelude::AppState,
    components::{lang::LanguageSwitcher, prelude::*},
};
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let state = use_context::<AppState>();
    rsx! {
        header { class: "flex flex-row justify-end items-center p-2 gap-2",

            ThemeSwitcher { theme: *state.theme.read() }
            LanguageSwitcher { language: *state.lang.read() }
        }
    }
}
