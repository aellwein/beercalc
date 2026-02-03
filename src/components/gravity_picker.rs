use crate::common::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn GravityPicker() -> Element {
    let mut state =
        use_synced_storage::<LocalStorage, AppState>(STORAGE_KEY.to_string(), AppState::default);
    rsx! {
        div { class: "flex flex-col gap-4 p-4",
            div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                span {
                    {t!("original-gravity")}
                    ": "
                }
                input {
                    r#type: "number",
                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300
                focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                    value: state.read().original_gravity.value().to_string(),
                    min: ".1",
                    step: ".1",
                    max: "60.0",
                    onchange: move |evt| {
                        let mut new_state = state.read().clone();
                        new_state.original_gravity = state
                            .read()
                            .original_gravity
                            .update_value(evt.value().parse::<f64>().unwrap_or(0.0));
                        state.set(new_state);
                    },
                }
            }
        }
    }
}
