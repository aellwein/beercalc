use crate::common::prelude::*;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn AlcoholCalc() -> Element {
    let state =
        use_synced_storage::<LocalStorage, AppState>(STORAGE_KEY.to_string(), AppState::default);
    rsx! {
        div { class: "container mx-auto max-w-6xl p-1",
            div { class: "flex flex-col gap-0",
                Header {}
                h1 { class: "text-2xl p-4", {t!("alcohol-calculator")} }
                UnitPicker { unit: state.read().chosen_unit.clone() }
                GravityPicker { gravity: state.read().chosen_unit.clone() }
            }
        }
    }
}
