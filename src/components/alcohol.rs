use crate::STATE;
use crate::common::prelude::*;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AlcoholCalculator() -> Element {
    rsx! {
        Fragment {
            Header { active: Route::AlcoholCalculator }
            div { class: "flex flex-col gap-4",
                div { class: TITLE_CLASS, {t!("alcohol_calculator")} }
                GravityUnitPicker {}
                GravityPicker {}
                div { class: "flex flex-row flex-wrap gap-4",
                    StandardCalculation {}
                    {
                        if matches!(STATE.read().original_gravity, Gravity::Brix(_)) {
                            rsx! {
                                TerrillCalculation {}
                            }
                        } else {
                            rsx! {}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderCalcResult(calc_result: AlcCalcResult) -> Element {
    rsx! {
        Fragment {
            div { class: "flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700",
                div { class: "text-left grow", {format!("{}:", t!("apparent_extract"))} }
                div { class: "text-right",
                    {format!(r#"{:.1}"#, calc_result.apparent_extract)}
                    {" °P"}
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700",
                div { class: "text-left grow", {format!("{}:", t!("real_extract"))} }
                div { class: "text-right",
                    {format!(r#"{:.1}"#, calc_result.real_extract)}
                    {" %"}
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700",
                div { class: "text-left grow", {format!("{}:", t!("apparent_attenuation"))} }
                div { class: "text-right",
                    {format!(r#"{:.1}"#, calc_result.apparent_attenuation)}
                    {" %"}
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700",
                div { class: "text-left grow", {format!("{}:", t!("real_attenuation"))} }
                div { class: "text-right",
                    {format!(r#"{:.1}"#, calc_result.real_attenuation)}
                    {" %"}
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700",
                div { class: "text-left grow", {format!("{}:", t!("alcohol_by_weight"))} }
                div { class: "text-right",
                    {format!(r#"{:.1}"#, calc_result.alcohol_by_weight)}
                    {" %"}
                }
            }
            div { class: "flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700",
                div { class: "text-left grow", {format!("{}:", t!("alcohol_by_volume"))} }
                div { class: "text-right",
                    {format!(r#"{:.1}"#, calc_result.alcohol_by_volume)}
                    {" %"}
                }
            }

        }
    }
}

#[component]
fn StandardCalculation() -> Element {
    if let Ok(calc_result) =
        calc_alcohol_standard_equation(&STATE.read().original_gravity, &STATE.read().final_gravity)
    {
        rsx! {
            Fragment {
                div { class: "flex flex-wrap flex-col gap-2 grow {CARD_CLASS}",
                    div { class: "{SECTION_TITLE_CLASS} text-center",
                        {
                            if matches!(STATE.read().original_gravity, Gravity::Brix(_)) {
                                t!("standard_equation")
                            } else {
                                t!("results")
                            }
                        }
                    }
                    RenderCalcResult { calc_result: calc_result.clone() }
                }
            }
        }
    } else {
        rsx! {
            Fragment {
                div { class: "flex flex-wrap flex-col gap-2 grow {CARD_CLASS}",
                    div { class: "{SECTION_TITLE_CLASS} text-center",
                        {
                            if matches!(STATE.read().original_gravity, Gravity::Brix(_)) {
                                t!("standard_equation")
                            } else {
                                t!("results")
                            }
                        }
                    }
                    div { class: "text-center grow text-danger table",
                        span { class: "table-cell align-middle", {t!("calculation_not_possible")} }
                    }
                }
            }
        }
    }
}

#[component]
fn TerrillCalculation() -> Element {
    if let Ok(calc_result) =
        calc_alcohol_terrill_equation(&STATE.read().original_gravity, &STATE.read().final_gravity)
    {
        rsx! {
            Fragment {
                div { class: "flex flex-wrap flex-col gap-2 grow {CARD_CLASS}",
                    div { class: "{SECTION_TITLE_CLASS} text-center", {t!("terrill_equation")} }
                    RenderCalcResult { calc_result: calc_result.clone() }
                }
            }
        }
    } else {
        rsx! {
            Fragment {
                div { class: "flex flex-wrap flex-col gap-2 grow {CARD_CLASS}",
                    div { class: "{SECTION_TITLE_CLASS} text-center", {t!("terrill_equation")} }
                    div { class: "text-center grow text-danger table",
                        span { class: "table-cell align-middle", {t!("calculation_not_possible")} }
                    }
                }
            }
        }
    }
}
