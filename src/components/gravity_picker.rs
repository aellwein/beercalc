use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn GravityPicker() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4 items-baseline",
            div { class: "flex flex-row gap-4 items-baseline",
                span { {t!("original_gravity")} }
                div {
                    input {
                        r#type: "number",
                        class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                        id: "og",
                        min: ".1",
                        max: "60.0",
                        step: "0.1",
                        value: format!("{:.1}", STATE.read().original_gravity.value().clone()),
                        onchange: move |evt| {
                            if let Ok(value) = evt.value().parse::<f64>() {
                                let mut new_state = STATE.read().clone();
                                new_state.original_gravity = new_state
                                    .original_gravity
                                    .new_with_updated_value(value);
                                *STATE.write() = new_state;
                            }
                        },
                    }
                }
                ShowUnits { gravity: STATE.read().original_gravity.clone() }
            }
            div { class: "flex flex-row gap-4 items-baseline",
                span { {t!("final_gravity")} }
                div {
                    input {
                        r#type: "number",
                        class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                        id: "fg",
                        min: ".1",
                        max: "60.0",
                        step: "0.1",
                        value: format!("{:.1}", STATE.read().final_gravity.value().clone()),
                        onchange: move |evt| {
                            if let Ok(value) = evt.value().parse::<f64>() {
                                let mut new_state = STATE.read().clone();
                                new_state.final_gravity = new_state
                                    .final_gravity
                                    .new_with_updated_value(value);
                                *STATE.write() = new_state;
                            }
                        },
                    }
                }
                ShowUnits { gravity: STATE.read().final_gravity.clone() }
            }
        }
    }
}
