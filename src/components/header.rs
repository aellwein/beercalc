use crate::components::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "flex flex-row justify-end items-center p-2 gap-2",
            ThemeSwitcher {}
            LanguageSwitcher {}
        }
    }
}
