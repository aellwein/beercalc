use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AlcoholCalc() -> Element {
    rsx! {
        div { class: "container mx-auto max-w-6xl p-1",
            div { class: "flex flex-col gap-0",
                Header {}
                h1 { class: "text-2xl p-4", {t!("alcohol-calculator")} }
                UnitPicker {}
                GravityPicker {}
            }
        }
    }
}
