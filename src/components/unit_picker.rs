use crate::common::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

fn change_to_brix() {
    let original_gravity = STATE.read().original_gravity.to_brix();
    let final_gravity = STATE.read().final_gravity.to_brix();
    let mut new_state = STATE.read().clone();
    new_state.original_gravity = original_gravity;
    new_state.final_gravity = final_gravity;
    *STATE.write() = new_state;
}

fn change_to_plato() {
    let original_gravity = STATE.read().original_gravity.to_plato();
    let final_gravity = STATE.read().final_gravity.to_plato();
    let mut new_state = STATE.read().clone();
    new_state.original_gravity = original_gravity;
    new_state.final_gravity = final_gravity;
    *STATE.write() = new_state;
}

#[component]
pub fn GravityUnitPicker() -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 shadow-md dark:shadow-neutral-600 items-baseline p-4",
            span { {format!("{}: ", t!("gravity_unit"))} }
            div {
                input {
                    r#type: "radio",
                    id: "brix",
                    class: "dark:bg-gray-700 dark:text-gray-300",
                    name: "gravity_unit",
                    checked: matches!(STATE.read().original_gravity, Gravity::Brix(_)),
                    onchange: move |_| change_to_brix(),
                }
                label { r#for: "brix", {format!(" {}", t!("brix"))} }
            }
            div {
                input {
                    r#type: "radio",
                    id: "plato",
                    class: "dark:bg-gray-700 dark:text-gray-300",
                    name: "gravity_unit",
                    checked: matches!(STATE.read().original_gravity, Gravity::Plato(_)),
                    onchange: move |_| change_to_plato(),
                }
                label { r#for: "plato", {format!(" {}", t!("plato"))} }
            }
        }
    }
}
