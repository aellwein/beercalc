use crate::common::prelude::*;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn BrewhouseEfficiencyCalculator() -> Element {
    let state = STATE.read().clone();
    let eff = calculate_brewhouse_efficiency(&state.original_gravity, &state.brewhouse);
    let gm = calculate_grain_mass_from_brewhouse(&state.brewhouse.grain_mass_from_brewhouse);

    rsx! {
        Fragment {
            Header { active: Route::BrewhouseEfficiencyCalculator }
            div { class: "text-2xl my-3", {t!("brewhouse_calculator")} }
            div { class: "flex flex-row gap-4 flex-wrap",
                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4 items-baseline grow",
                    div { class: "text-xl", {t!("brewhouse_efficiency")} }
                    div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                        div { {t!("grain_mass")} }
                        input {
                            class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                            r#type: "number",
                            min: ".1",
                            max: "100000",
                            step: ".1",
                            value: format!("{:.1}", state.brewhouse.grain_mass),
                            onchange: move |evt| {
                                if let Ok(value) = evt.value().parse::<f64>() {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.grain_mass = value.max(0.1);
                                    *STATE.write() = new_state;
                                }
                            },
                        }
                        select {
                            class: "p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300",
                            onchange: move |evt| {
                                let mut new_state = STATE.read().clone();
                                new_state.brewhouse.grain_mass_unit = evt.value().into();
                                *STATE.write() = new_state;
                            },
                            {[MassUnit::Kilogram, MassUnit::Gram].iter().map(|unit| rsx! {
                                option {
                                    value: "{unit.translator_key()}",
                                    key: "{unit.translator_key()}",
                                    selected: {unit == &STATE.read().brewhouse.grain_mass_unit},
                                    {t!(&unit.translator_key())}
                                }
                            })}
                        }
                    }
                    div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                        div { {t!("original_gravity")} }
                        input {
                            class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                            r#type: "number",
                            min: ".1",
                            max: "60",
                            step: ".1",
                            value: format!("{:.1}", state.original_gravity.value()),
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
                        div { {t!("wort_volume")} }
                        input {
                            class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                            r#type: "number",
                            min: ".1",
                            max: "1000",
                            step: ".1",
                            value: format!("{:.1}", state.brewhouse.wort_volume),
                            onchange: move |evt| {
                                if let Ok(value) = evt.value().parse::<f64>() {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.wort_volume = value.max(0.1);
                                    *STATE.write() = new_state;
                                }
                            },
                        }
                        div { {t!("liter")} }
                    }
                    div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                        div { {t!("volume_measured_at")} }
                        label {
                            input {
                                r#type: "radio",
                                class: "dark:bg-gray-700 dark:text-gray-300",
                                name: "vma",
                                checked: matches!(state.brewhouse.volume_measured_at, VolumeMeasuredAt::HundredDegreesCelsius),
                                onchange: move |_| {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.volume_measured_at =
                                        VolumeMeasuredAt::HundredDegreesCelsius;
                                    *STATE.write() = new_state;
                                },
                            }
                            " 100 °C"
                        }
                        label {
                            input {
                                r#type: "radio",
                                class: "dark:bg-gray-700 dark:text-gray-300",
                                name: "vma",
                                checked: matches!(state.brewhouse.volume_measured_at, VolumeMeasuredAt::TwentyDegreesCelsius),
                                onchange: move |_| {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.volume_measured_at =
                                        VolumeMeasuredAt::TwentyDegreesCelsius;
                                    *STATE.write() = new_state;
                                },
                            }
                            " 20 °C"
                        }
                    }
                    div { strong { {format!("{}: {:.1} %", t!("brewhouse_efficiency"), eff)} } }
                }

                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4 items-baseline grow",
                    div { class: "text-xl", {t!("grain_mass_relative_to_brewhouse_efficiency")} }
                    div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                        div { {t!("brewhouse_efficiency")} }
                        input {
                            class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                            r#type: "number",
                            min: ".1",
                            max: "100",
                            step: ".1",
                            value: format!(
                                "{:.1}",
                                state.brewhouse.grain_mass_from_brewhouse.brewhouse_efficiency
                            ),
                            onchange: move |evt| {
                                if let Ok(value) = evt.value().parse::<f64>() {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.grain_mass_from_brewhouse
                                        .brewhouse_efficiency = value.max(0.1);
                                    *STATE.write() = new_state;
                                }
                            },
                        }
                        "%"
                    }
                    div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                        div { {t!("original_gravity")} }
                        input {
                            class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                            r#type: "number",
                            min: ".1",
                            max: "60",
                            step: ".1",
                            value: format!(
                                "{:.1}",
                                state
                                    .brewhouse
                                    .grain_mass_from_brewhouse
                                    .original_gravity
                                    .value()
                            ),
                            onchange: move |evt| {
                                if let Ok(value) = evt.value().parse::<f64>() {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.grain_mass_from_brewhouse.original_gravity =
                                        new_state
                                            .brewhouse
                                            .grain_mass_from_brewhouse
                                            .original_gravity
                                            .new_with_updated_value(value);
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
                                    new_state.brewhouse.grain_mass_from_brewhouse.original_gravity =
                                        new_state
                                            .brewhouse
                                            .grain_mass_from_brewhouse
                                            .original_gravity
                                            .to_plato();
                                } else if unit == "brix" {
                                    new_state.brewhouse.grain_mass_from_brewhouse.original_gravity =
                                        new_state
                                            .brewhouse
                                            .grain_mass_from_brewhouse
                                            .original_gravity
                                            .to_brix();
                                }
                                *STATE.write() = new_state;
                            },
                            {[Gravity::Plato(0.0), Gravity::Brix(0.0)].iter().map(|g| rsx! {
                                option {
                                    value: "{g.translator_key()}",
                                    key: "{g.translator_key()}",
                                    selected: {matches!((&state.brewhouse.grain_mass_from_brewhouse.original_gravity, g), (Gravity::Plato(_), Gravity::Plato(_)) | (Gravity::Brix(_), Gravity::Brix(_)))},
                                    {t!(&g.translator_key())}
                                }
                            })}
                        }
                    }
                    div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                        div { {t!("wort_volume")} }
                        input {
                            class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                            r#type: "number",
                            min: ".1",
                            max: "1000",
                            step: ".1",
                            value: format!(
                                "{:.1}",
                                state.brewhouse.grain_mass_from_brewhouse.wort_volume
                            ),
                            onchange: move |evt| {
                                if let Ok(value) = evt.value().parse::<f64>() {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.grain_mass_from_brewhouse.wort_volume =
                                        value.max(0.1);
                                    *STATE.write() = new_state;
                                }
                            },
                        }
                        div { {t!("liter")} }
                    }
                    div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                        div { {t!("volume_measured_at")} }
                        label {
                            input {
                                r#type: "radio",
                                class: "dark:bg-gray-700 dark:text-gray-300",
                                name: "vma2",
                                checked: matches!(state.brewhouse.grain_mass_from_brewhouse.volume_measured_at, VolumeMeasuredAt::HundredDegreesCelsius),
                                onchange: move |_| {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.grain_mass_from_brewhouse.volume_measured_at =
                                        VolumeMeasuredAt::HundredDegreesCelsius;
                                    *STATE.write() = new_state;
                                },
                            }
                            " 100 °C"
                        }
                        label {
                            input {
                                r#type: "radio",
                                class: "dark:bg-gray-700 dark:text-gray-300",
                                name: "vma2",
                                checked: matches!(state.brewhouse.grain_mass_from_brewhouse.volume_measured_at, VolumeMeasuredAt::TwentyDegreesCelsius),
                                onchange: move |_| {
                                    let mut new_state = STATE.read().clone();
                                    new_state.brewhouse.grain_mass_from_brewhouse.volume_measured_at =
                                        VolumeMeasuredAt::TwentyDegreesCelsius;
                                    *STATE.write() = new_state;
                                },
                            }
                            " 20 °C"
                        }
                    }
                    div { strong { {format!("{}: {:.1} {}", t!("grain_mass"), gm, t!("kg"))} } }
                }
            }
        }
    }
}
