use crate::common::prelude::*;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn IbuCalculator() -> Element {
    rsx! {
        Fragment {
            Header { active: Route::IbuCalculator }
            div { class: "text-2xl my-3", {t!("ibu_calculator")} }
            IbuPreset {}
        }
    }
}

#[component]
fn IbuPreset() -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 items-baseline flex-wrap",
            div { {t!("original_gravity")} }
            input {
                class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                r#type: "number",
                min: "0.1",
                step: ".1",
                max: "60.0",
                value: format!("{:.1}", STATE.read().original_gravity.value()),
            }
            select {
                class: "p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300",
                onchange: move |evt| {},
                {[Gravity::Plato(0.0), Gravity::Brix(0.0)].iter().map(|g| rsx! {
                    option {
                        value: {t!(& g.translator_key())},
                        key: "{t!(& g.translator_key())}",
                        selected: {matches!(&STATE.read().original_gravity, g)},
                        {t!(& g.translator_key())}
                    }
                })}
            }
            ShowUnits { gravity: STATE.read().original_gravity.clone() }
        }
    }
}
