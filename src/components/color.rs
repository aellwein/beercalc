use crate::common::prelude::*;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn BeerColorCalculator() -> Element {
    let ebc = calculate_ebc(
        &STATE.read().original_gravity,
        &STATE.read().grain.malt,
        STATE.read().ibu.boiling,
    );
    rsx! {
        Fragment {
            Header { active: Route::BeerColorCalculator }
            div { class: "text-2xl my-3", {t!("color_calculator")} }
            div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
                div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                    div { {t!("original_gravity")} }
                    input {
                        class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                        r#type: "number",
                        min: ".1",
                        max: "40",
                        step: ".1",
                        value: format!("{:.1}", STATE.read().original_gravity.value()),
                        onchange: move |evt| {
                            if let Ok(value) = evt.value().parse::<f64>() {
                                let mut new_state = STATE.read().clone();
                                new_state.original_gravity =
                                    new_state.original_gravity.new_with_updated_value(value);
                                *STATE.write() = new_state;
                            }
                        },
                    }
                    select {
                        class: "p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300",
                        onchange: move |evt| {
                            let unit = evt.value();
                            let mut new_state = STATE.read().clone();
                            if unit == "plato" {
                                new_state.original_gravity = new_state.original_gravity.to_plato();
                                new_state.final_gravity = new_state.final_gravity.to_plato();
                            } else if unit == "brix" {
                                new_state.original_gravity = new_state.original_gravity.to_brix();
                                new_state.final_gravity = new_state.final_gravity.to_brix();
                            }
                            *STATE.write() = new_state;
                        },
                        {[Gravity::Plato(0.0), Gravity::Brix(0.0)].iter().map(|g| rsx! {
                            option {
                                value: "{g.translator_key()}",
                                key: "{g.translator_key()}",
                                selected: {matches!((&STATE.read().original_gravity, g), (Gravity::Plato(_), Gravity::Plato(_)) | (Gravity::Brix(_), Gravity::Brix(_)))},
                                {t!(&g.translator_key())}
                            }
                        })}
                    }
                }
                div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                    div { {t!("boiling_time")} }
                    input {
                        class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                        r#type: "number",
                        min: "1",
                        max: "600",
                        step: "1",
                        value: format!("{:.0}", STATE.read().ibu.boiling),
                        onchange: move |evt| {
                            if let Ok(value) = evt.value().parse::<f64>() {
                                let boiling = value.clamp(1.0, 600.0);
                                let mut new_state = STATE.read().clone();
                                new_state.ibu.boiling = boiling;
                                for hop in &mut new_state.ibu.hops {
                                    if hop.boil_time > boiling {
                                        hop.boil_time = boiling;
                                    }
                                }
                                *STATE.write() = new_state;
                            }
                        },
                    }
                    div { {t!("minutes")} }
                }
            }
            MaltAdder {}
            EbcColorDisplay { ebc: ebc as f64 }
        }
    }
}

#[component]
fn MaltAdder() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
            {
                STATE.read().grain.malt.iter().enumerate().map(|(idx, grain)| {
                    rsx! {
                        div { key: "malt-{idx}", class: "flex flex-col gap-3 border-b border-neutral-200 pb-2 dark:border-neutral-700",
                            div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                                div { {format!("{} #{}", t!("malt"), idx + 1)} }
                                button {
                                    class: "py-2 px-2 bg-red-300 dark:bg-red-900 dark:hover:bg-red-700 text-white hover:bg-red-600",
                                    onclick: move |_| {
                                        let mut new_state = STATE.read().clone();
                                        if idx < new_state.grain.malt.len() {
                                            new_state.grain.malt.remove(idx);
                                            *STATE.write() = new_state;
                                        }
                                    },
                                    "X"
                                }
                            }
                            div { class: "flex flex-row items-baseline gap-4",
                                div { {t!("mass")} }
                                input {
                                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                                    r#type: "number",
                                    min: "0",
                                    max: "100000",
                                    step: ".1",
                                    value: format!("{:.1}", grain.amount),
                                    onchange: move |evt| {
                                        if let Ok(value) = evt.value().parse::<f64>() {
                                            let mut new_state = STATE.read().clone();
                                            if let Some(item) = new_state.grain.malt.get_mut(idx) {
                                                item.amount = value.max(0.0);
                                                *STATE.write() = new_state;
                                            }
                                        }
                                    },
                                }
                            }
                            div { class: "flex flex-row items-baseline gap-4",
                                div { {t!("mass_unit")} }
                                select {
                                    class: "p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300",
                                    onchange: move |evt| {
                                        let unit: MassUnit = evt.value().into();
                                        let mut new_state = STATE.read().clone();
                                        if let Some(item) = new_state.grain.malt.get_mut(idx) {
                                            item.mass_unit = unit;
                                            *STATE.write() = new_state;
                                        }
                                    },
                                    {[MassUnit::Kilogram, MassUnit::Gram].iter().map(|unit| rsx! {
                                        option {
                                            value: "{unit.translator_key()}",
                                            key: "{unit.translator_key()}",
                                            selected: {unit == &grain.mass_unit},
                                            {t!(&unit.translator_key())}
                                        }
                                    })}
                                }
                            }
                            div { class: "flex flex-row items-baseline gap-4",
                                div { {t!("color_ebc")} }
                                input {
                                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                                    r#type: "number",
                                    min: "1",
                                    max: "10000",
                                    step: "1",
                                    value: format!("{:.0}", grain.color),
                                    onchange: move |evt| {
                                        if let Ok(value) = evt.value().parse::<f64>() {
                                            let mut new_state = STATE.read().clone();
                                            if let Some(item) = new_state.grain.malt.get_mut(idx) {
                                                item.color = value.max(1.0);
                                                *STATE.write() = new_state;
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                })
            }
            div {
                button {
                    class: "py-2 px-4 bg-yellow-600 dark:bg-yellow-700 text-white dark:text-gray-100 rounded-lg shadow-md dark:shadow-slate-600 hover:bg-yellow-800 dark:hover:bg-yellow-500 dark:hover:text-gray-800",
                    onclick: move |_| {
                        let mut new_state = STATE.read().clone();
                        new_state.grain.malt.push(Malt {
                            amount: 5.0,
                            mass_unit: MassUnit::Kilogram,
                            color: 8.0,
                        });
                        *STATE.write() = new_state;
                    },
                    {t!("add_malt")}
                }
            }
        }
    }
}

#[component]
pub fn EbcColorDisplay(ebc: f64) -> Element {
    let ebc_value = ebc.round() as i32;
    let class_name = if ebc_value < 100 {
        format!("ebc-{}", ebc_value.max(0))
    } else {
        "ebc-100".to_string()
    };
    rsx! {
        div { class: "flex flex-row shadow-md dark:shadow-slate-600 p-4 gap-4 items-baseline",
            div { class: "my-auto grow", strong { {format!("{} EBC", ebc_value)} } }
            div { class: "{class_name} grow", style: "height: 100px" }
        }
    }
}
