use crate::common::prelude::*;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

fn update_ibu<F>(mutate: F)
where
    F: FnOnce(&mut Ibu),
{
    let mut new_state = STATE.read().clone();
    mutate(&mut new_state.ibu);
    *STATE.write() = new_state;
}

fn calc_hop_ibu(original_gravity: &Gravity, ibu: &Ibu, hops: &Hops) -> f64 {
    let iso_speed_factor = 0.046 * (0.031 * ibu.flameout_temp).exp();
    let add_iso_time = iso_speed_factor * ibu.flameout;
    let hops_factor = match hops.form {
        HopsForm::Whole => 1.0,
        HopsForm::Pellets => 1.1,
    };

    hops_factor
        * ((hops.amount * hops.alpha * 10.0 / ibu.volume)
            * (1.65 * 0.000125_f64.powf(0.004 * original_gravity.to_plato().value()))
            * (1.0 - (-0.04 * (hops.boil_time + add_iso_time)).exp())
            / 4.15)
}

fn total_ibu(original_gravity: &Gravity, ibu: &Ibu) -> f64 {
    ibu.hops
        .iter()
        .map(|hop| calc_hop_ibu(original_gravity, ibu, hop))
        .sum()
}

#[component]
pub fn IbuCalculator() -> Element {
    rsx! {
        Fragment {
            Header { active: Route::IbuCalculator }
            div { class: "flex flex-col gap-4 dark:text-gray-400",
                div { class: "text-2xl my-3", {t!("ibu_calculator")} }
                IbuPreset {}
                HopsAdder {}
            }
        }
    }
}

#[component]
fn IbuPreset() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
            div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                div { {t!("original_gravity")} }
                input {
                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                    r#type: "number",
                    min: "0.1",
                    step: ".1",
                    max: "60.0",
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
                ShowUnits { gravity: STATE.read().original_gravity.clone() }
            }
            div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                div { {t!("boiling_time")} }
                input {
                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                    r#type: "number",
                    min: "1",
                    step: "1",
                    max: "600",
                    value: format!("{}", STATE.read().ibu.boiling),
                    onchange: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            let boiling = value.clamp(1.0, 600.0);
                            update_ibu(|ibu| {
                                ibu.boiling = boiling;
                                for hop in &mut ibu.hops {
                                    if hop.boil_time > boiling {
                                        hop.boil_time = boiling;
                                    }
                                }
                            });
                        }
                    },
                }
                div { {t!("minutes")} }
            }
            div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                div { {t!("volume")} }
                input {
                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                    r#type: "number",
                    min: "1",
                    step: "1",
                    max: "500",
                    value: format!("{:.0}", STATE.read().ibu.volume),
                    onchange: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            update_ibu(|ibu| ibu.volume = value.max(1.0));
                        }
                    },
                }
                div { {t!("liter")} }
            }
            div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                div { {t!("flameout_time")} }
                input {
                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                    r#type: "number",
                    min: "0",
                    step: "1",
                    max: "90",
                    value: format!("{:.0}", STATE.read().ibu.flameout),
                    onchange: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            update_ibu(|ibu| ibu.flameout = value.max(0.0));
                        }
                    },
                }
                div { {t!("minutes")} }
            }
            div { class: "flex flex-row gap-4 items-baseline flex-wrap",
                div { {t!("flameout_temp")} }
                input {
                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                    r#type: "number",
                    min: "0",
                    step: "1",
                    max: "100",
                    value: format!("{:.0}", STATE.read().ibu.flameout_temp),
                    onchange: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            update_ibu(|ibu| ibu.flameout_temp = value.clamp(0.0, 100.0));
                        }
                    },
                }
                div { "°C" }
            }
        }
    }
}

#[component]
fn HopsAdder() -> Element {
    let state = STATE.read().clone();
    let total = total_ibu(&state.original_gravity, &state.ibu);

    rsx! {
        div { class: "flex flex-col gap-4 p-4 shadow-md dark:shadow-slate-600",
            {
                state.ibu.hops.iter().enumerate().map(|(idx, hops)| {
                    let bitterness = calc_hop_ibu(&state.original_gravity, &state.ibu, hops);
                    rsx! {
                        div { key: "hops-{idx}", class: "flex flex-col gap-4",
                            div { class: "flex flex-row items-baseline gap-4",
                                div { class: "underline text-right grow", {format!("{} #{}", t!("hops_addition"), idx + 1)} }
                                button {
                                    class: "py-2 px-2 bg-red-300 dark:bg-red-900 dark:hover:bg-red-700 text-white hover:bg-red-600",
                                    onclick: move |_| {
                                        update_ibu(|ibu| {
                                            if idx < ibu.hops.len() {
                                                ibu.hops.remove(idx);
                                            }
                                        });
                                    },
                                    "X"
                                }
                            }
                            div { class: "flex flex-row items-baseline gap-4 flex-wrap",
                                div { {t!("form")} }
                                select {
                                    class: "p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300",
                                    onchange: move |evt| {
                                        let form: HopsForm = evt.value().into();
                                        update_ibu(|ibu| {
                                            if let Some(item) = ibu.hops.get_mut(idx) {
                                                item.form = form;
                                            }
                                        });
                                    },
                                    {[HopsForm::Whole, HopsForm::Pellets].iter().map(|form| rsx! {
                                        option {
                                            key: "{form.translator_key()}",
                                            value: "{form.translator_key()}",
                                            selected: {form == &hops.form},
                                            {t!(&form.translator_key())}
                                        }
                                    })}
                                }
                            }
                            div { class: "flex flex-row items-baseline gap-4 flex-wrap",
                                div { {t!("amount")} }
                                input {
                                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                                    r#type: "number",
                                    min: "0",
                                    max: "5000",
                                    step: ".1",
                                    value: format!("{:.1}", hops.amount),
                                    onchange: move |evt| {
                                        if let Ok(value) = evt.value().parse::<f64>() {
                                            update_ibu(|ibu| {
                                                if let Some(item) = ibu.hops.get_mut(idx) {
                                                    item.amount = value.max(0.0);
                                                }
                                            });
                                        }
                                    },
                                }
                            }
                            div { class: "flex flex-row items-baseline gap-4 flex-wrap",
                                div { {t!("alpha_acid")} }
                                input {
                                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                                    r#type: "number",
                                    min: "0",
                                    max: "90",
                                    step: ".1",
                                    value: format!("{:.1}", hops.alpha),
                                    onchange: move |evt| {
                                        if let Ok(value) = evt.value().parse::<f64>() {
                                            update_ibu(|ibu| {
                                                if let Some(item) = ibu.hops.get_mut(idx) {
                                                    item.alpha = value.clamp(0.0, 90.0);
                                                }
                                            });
                                        }
                                    },
                                }
                            }
                            div { class: "flex flex-row items-baseline gap-4 flex-wrap",
                                div { {t!("boil_time")} }
                                input {
                                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                                    r#type: "number",
                                    min: "0",
                                    max: "{state.ibu.boiling}",
                                    step: "1",
                                    value: format!("{:.0}", hops.boil_time),
                                    onchange: move |evt| {
                                        if let Ok(value) = evt.value().parse::<f64>() {
                                            let max = STATE.read().ibu.boiling;
                                            update_ibu(|ibu| {
                                                if let Some(item) = ibu.hops.get_mut(idx) {
                                                    item.boil_time = value.clamp(0.0, max);
                                                }
                                            });
                                        }
                                    },
                                }
                            }
                            div { class: "flex flex-row items-baseline gap-4",
                                div { class: "grow dark:text-gray-200", {t!("bitterness")} }
                                div { class: "text-right dark:text-gray-200", {format!("{:.1} IBU", bitterness)} }
                            }
                        }
                    }
                })
            }
            div { class: "flex flex-row gap-4",
                div { class: "grow dark:text-gray-200", strong { {t!("total_bitterness")} } }
                div { class: "text-right dark:text-gray-200", strong { {format!("{:.1} IBU", total)} } }
            }
            div { class: "mr-auto",
                button {
                    class: "py-2 px-4 text-white dark:text-gray-200 bg-green-500 dark:bg-green-900 rounded-lg shadow-md dark:shadow-slate-600 hover:bg-green-700 dark:hover:bg-green-800",
                    onclick: move |_| {
                        let boiling = STATE.read().ibu.boiling;
                        update_ibu(|ibu| {
                            ibu.hops.push(Hops {
                                form: HopsForm::Pellets,
                                amount: 5.0,
                                alpha: 7.0,
                                boil_time: boiling,
                                ibu: 0.0,
                            });
                        });
                    },
                    {t!("add_hops")}
                }
            }
        }
    }
}
