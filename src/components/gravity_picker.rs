use crate::STATE;
use crate::common::prelude::*;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn GravityPicker() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 items-baseline {CARD_CLASS}",
            div { class: "flex flex-row flex-wrap gap-4 items-baseline",
                span { {t!("original_gravity")} }
                div {
                    input {
                        r#type: "number",
                        class: INPUT_CLASS,
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
            div { class: "flex flex-row flex-wrap gap-4 items-baseline",
                span { {t!("final_gravity")} }
                div {
                    input {
                        r#type: "number",
                        class: INPUT_CLASS,
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
