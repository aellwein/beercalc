use crate::common::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn UnitPicker() -> Element {
    let mut state =
        use_synced_storage::<LocalStorage, AppState>(STORAGE_KEY.to_string(), AppState::default);
    rsx! {
        div { class: "flex flex-row gap-4 p-4",
            span {
                {t!("gravity-unit")}
                ":"
            }
            div {
                input {
                    r#type: "radio",
                    id: "brix",
                    name: "unit",
                    class: "dark:bg-gray-700 dark:text-gray-300",
                    checked: state.read().chosen_unit.eq(&Gravity::Brix(0.0)),
                    onchange: move |_| {
                        let mut new_state = state.read().clone();
                        new_state.chosen_unit = Gravity::Brix(0.0);
                        let og = new_state.original_gravity;
                        new_state.original_gravity = og.to_brix();
                        state.set(new_state);
                    },
                }
                label { r#for: "brix",
                    " "
                    {t!("brix")}
                }
            }
            div {
                input {
                    r#type: "radio",
                    id: "plato",
                    name: "unit",
                    class: "dark:bg-gray-700 dark:text-gray-300",
                    checked: state.read().chosen_unit.eq(&Gravity::Plato(0.0)),
                    onchange: move |_| {
                        let mut new_state = state.read().clone();
                        new_state.chosen_unit = Gravity::Plato(0.0);
                        let og = new_state.original_gravity;
                        new_state.original_gravity = og.to_plato();
                        *state.write() = new_state;
                    },
                }
                label { r#for: "plato",
                    " "
                    {t!("plato")}
                }
            }
        }
    }
}
