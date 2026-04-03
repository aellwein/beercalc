use crate::common::prelude::*;
use crate::components::lang;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn GravityUnitPicker(current_unit: Gravity) -> Element {
    let mut state =
        use_synced_storage::<LocalStorage, CalcState>(STATE_NAME.to_string(), CalcState::default);
    rsx! {
        div { class: "flex flex-row gap-4 shadow-md dark:shadow-neutral-600 p-4",
            span { {t!("gravity_unit")} }
            div {
                input {
                    r#type: "radio",
                    id: "brix",
                    class: "dark:bg-gray-700 dark:text-gray-300",
                    name: "gravity_unit",
                    checked: matches!(current_unit, Gravity::Brix(_)),
                    onchange: move |_| {
                        let mut new_state = state.read().clone();
                        new_state.chosen_unit = Gravity::Brix(0.0);
                        state.set(new_state);
                    },
                }
                label { r#for: "brix", {format!(" {}", t!("brix"))} }
            }
            div {
                input {
                    r#type: "radio",
                    id: "plato",
                    class: "dark:bg-gray-700 dark:text-gray-300",
                    name: "gravity_unit",
                    checked: matches!(current_unit, Gravity::Plato(_)),
                    onchange: move |_| {
                        let mut new_state = state.read().clone();
                        new_state.chosen_unit = Gravity::Plato(0.0);
                        state.set(new_state);
                    },
                }
                label { r#for: "plato", {format!(" {}", t!("plato"))} }
            }
        }
    }
}
