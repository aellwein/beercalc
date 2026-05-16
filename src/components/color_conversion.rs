use crate::common::prelude::*;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ColorConversionCalculator() -> Element {
    let color = STATE.read().color_conversion.color.clone();
    let min_value = match color {
        ColorUnit::Ebc(_) => 0.5,
        ColorUnit::Srm(_) => 0.3,
        ColorUnit::Lovibond(_) => 0.7,
    };

    rsx! {
        Fragment {
            Header { active: Route::ColorConversionCalculator }
            div { class: "text-2xl my-3", {t!("color_conversion")} }
            div { class: "flex flex-row gap-4 shadow-md dark:shadow-slate-600 p-4 items-baseline flex-wrap",
                span { {t!("malt_color")} }
                input {
                    r#type: "number",
                    min: "{min_value}",
                    max: "200",
                    step: ".1",
                    value: format!("{:.1}", color.value()),
                    class: "border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300",
                    onchange: move |evt| {
                        if let Ok(value) = evt.value().parse::<f64>() {
                            let mut new_state = STATE.read().clone();
                            new_state.color_conversion.color =
                                new_state.color_conversion.color.update_value(value);
                            *STATE.write() = new_state;
                        }
                    },
                }
                select {
                    class: "p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300",
                    onchange: move |evt| {
                        let mut new_state = STATE.read().clone();
                        let v = new_state.color_conversion.color.value();
                        new_state.color_conversion.color = match evt.value().as_str() {
                            "ebc" => ColorUnit::Ebc(v),
                            "srm" => ColorUnit::Srm(v),
                            "lovibond" => ColorUnit::Lovibond(v),
                            _ => new_state.color_conversion.color.clone(),
                        };
                        *STATE.write() = new_state;
                    },
                    {[ColorUnit::Ebc(0.0), ColorUnit::Srm(0.0), ColorUnit::Lovibond(0.0)].iter().map(|unit| rsx! {
                        option {
                            value: "{unit.translator_key()}",
                            key: "{unit.translator_key()}",
                            selected: {unit.match_unit(&STATE.read().color_conversion.color)},
                            {t!(&unit.translator_key())}
                        }
                    })}
                }
                span {
                    {
                        let c = STATE.read().color_conversion.color.clone();
                        match c {
                            ColorUnit::Ebc(_) => format!(
                                "{:.1} {} / {:.1} {}",
                                c.to_srm().value(),
                                t!("srm"),
                                c.to_lovibond().value(),
                                t!("lovibond")
                            ),
                            ColorUnit::Srm(_) => format!(
                                "{:.1} {} / {:.1} {}",
                                c.to_ebc().value(),
                                t!("ebc"),
                                c.to_lovibond().value(),
                                t!("lovibond")
                            ),
                            ColorUnit::Lovibond(_) => format!(
                                "{:.1} {} / {:.1} {}",
                                c.to_ebc().value(),
                                t!("ebc"),
                                c.to_srm().value(),
                                t!("srm")
                            ),
                        }
                    }
                }
            }
            EbcColorDisplay { ebc: STATE.read().color_conversion.color.to_ebc().value() }
        }
    }
}
