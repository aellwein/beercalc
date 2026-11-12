use crate::common::prelude::*;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

fn update_carbonation<F>(mutate: F)
where
    F: FnOnce(&mut Carbonation),
{
    let mut new_state = STATE.read().clone();
    mutate(&mut new_state.carbonation);
    *STATE.write() = new_state;
}

#[component]
pub fn CarbonationCalculator() -> Element {
    let c = STATE.read().carbonation.clone();

    let spunding_text = match spunding_pressure(c.target_co2, c.spunding_temperature) {
        Ok(pressure) => format!("{:.2} bar", pressure),
        Err(_) => t!("co2_too_low").to_string(),
    };
    let dissolved_text = format!(
        "{:.2} g/L",
        dissolved_co2(c.vessel_pressure, c.co2_temperature)
    );

    rsx! {
        Fragment {
            Header { active: Route::CarbonationCalculator }
            div { class: "flex flex-col gap-4 dark:text-gray-400",
                div { class: "text-2xl my-3", {t!("carbonation_calculator")} }
                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
                    {number_field(
                        t!("batch_volume").to_string(),
                        c.batch_volume,
                        (1.0, 1000.0, 0.5),
                        "L",
                        |v| update_carbonation(|c| c.batch_volume = v),
                    )}
                }
                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
                    div { class: "text-xl", {t!("spunding_pressure")} }
                    {number_field(
                        t!("co2_target").to_string(),
                        c.target_co2,
                        (0.0, 10.0, 0.1),
                        "g/L",
                        |v| update_carbonation(|c| c.target_co2 = v),
                    )}
                    {number_field(
                        t!("temperature_in_vessel").to_string(),
                        c.spunding_temperature,
                        (0.0, 30.0, 0.5),
                        "°C",
                        |v| update_carbonation(|c| c.spunding_temperature = v),
                    )}
                    div { class: "flex flex-row gap-4 items-baseline",
                        div { class: "grow dark:text-gray-200", strong { {t!("spunding_pressure")} } }
                        div { class: "text-right dark:text-gray-200", strong { {spunding_text} } }
                    }
                }
                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
                    div { class: "text-xl", {t!("co2_from_pressure")} }
                    {number_field(
                        t!("vessel_pressure").to_string(),
                        c.vessel_pressure,
                        (0.0, 10.0, 0.1),
                        "bar",
                        |v| update_carbonation(|c| c.vessel_pressure = v),
                    )}
                    {number_field(
                        t!("temperature_in_vessel").to_string(),
                        c.co2_temperature,
                        (0.0, 30.0, 0.5),
                        "°C",
                        |v| update_carbonation(|c| c.co2_temperature = v),
                    )}
                    div { class: "flex flex-row gap-4 items-baseline",
                        div { class: "grow dark:text-gray-200", strong { {t!("co2_dissolved")} } }
                        div { class: "text-right dark:text-gray-200", strong { {dissolved_text} } }
                    }
                }
            }
        }
    }
}
