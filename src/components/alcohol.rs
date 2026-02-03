use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::header::Header;

#[component]
pub fn AlcoholCalc() -> Element {
    rsx! {
        div { class: "container mx-auto max-w-6xl p-1 border border-black",
            Header {}
            div {
                h1 { class: "text-xl", {t!("alcohol-calculator")} }

            }
        }
    }
}
