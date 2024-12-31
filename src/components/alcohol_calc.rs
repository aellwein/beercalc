use crate::common::prelude::*;
use crate::components::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn AlcoholCalculator() -> Html {
    let (state, _) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    html! {
        <>
            <Header active={Route::AlcoholCalculator} />
            <div class="flex flex-col gap-4 dark:text-gray-400">
                <div class="text-2xl my-3">{t.t("alcohol calculator", &state.language)}</div>
                <UnitPicker />
                <GravityPicker />
                <div class="flex flex-row flex-wrap gap-4">
                    <AlcCalc equation={EquationType::Standard} />
                    { if matches!(state.chosen_unit, Gravity::Brix(_)) {
                        html! {
                            <AlcCalc equation={EquationType::Terrill} />
                        }
                    } else { html! {} }
                    }
                </div>
            </div>
            <Footer />
        </>
    }
}

#[derive(Clone, PartialEq)]
enum EquationType {
    Standard,
    Terrill,
}

#[derive(Properties, Clone, PartialEq)]
struct CalcResultProps {
    equation: EquationType,
}

#[function_component]
fn AlcCalc(props: &CalcResultProps) -> Html {
    let (state, _) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    let title = match props.equation {
        EquationType::Standard => {
            if matches!(state.chosen_unit, Gravity::Brix(_)) {
                t.t("standard equation", &state.language)
            } else {
                t.t("results", &state.language)
            }
        }
        EquationType::Terrill => t.t("terrill equation", &state.language),
    };
    let calc_result = match props.equation {
        EquationType::Standard => {
            calc_alcohol_standard_equation(&state.original_gravity, &state.final_gravity)
        }
        EquationType::Terrill => {
            calc_alcohol_terrill_equation(&state.original_gravity, &state.final_gravity)
        }
    };
    html! {
        <div class="flex flex-wrap flex-col shadow-md p-4 gap-2 flex-grow">
            <div class="text-2xl text-center my-3">{title}</div>
            {
                if let Ok(result) = calc_result {
                    html! {
                        <>
                        <div class="flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700 dark:hover:border-neutral-500">
                            <div class="text-left flex-grow">{t.t("apparent extract", &state.language)}{":"}</div>
                            <div class="text-right">{format!(r#"{:.1}"#, result.apparent_extract)}{" °P"}</div>
                        </div>
                        <div class="flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700 dark:hover:border-neutral-500">
                            <div class="text-left flex-grow">{t.t("real extract", &state.language)}{":"}</div>
                            <div class="text-right">{format!(r#"{:.1}"#, result.real_extract)}{" %"}</div>
                        </div>
                        <div class="flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700 dark:hover:border-neutral-500">
                            <div class="text-left flex-grow">{t.t("apparent attenuation", &state.language)}{":"}</div>
                            <div class="text-right">{format!(r#"{:.1}"#, result.apparent_attenuation)}{" %"}</div>
                        </div>
                        <div class="flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700 dark:hover:border-neutral-500">
                            <div class="text-left flex-grow">{t.t("real attenuation", &state.language)}{":"}</div>
                            <div class="text-right">{format!(r#"{:.1}"#, result.real_attenuation)}{" %"}</div>
                        </div>
                        <div class="flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700 dark:hover:border-neutral-500">
                            <div class="text-left flex-grow">{t.t("alcohol by weight", &state.language)}{":"}</div>
                            <div class="text-right">{format!(r#"{:.1}"#, result.alcohol_by_weight)}{" %"}</div>
                        </div>
                        <div class="flex flex-row flex-wrap gap-2 hover:border-b hover:border-neutral-700 dark:hover:border-neutral-500">
                            <div class="text-left flex-grow">{t.t("alcohol by volume", &state.language)}{":"}</div>
                            <div class="text-right">{format!(r#"{:.1}"#, result.alcohol_by_volume)}{" %"}</div>
                        </div>

                        </>
                    }
                } else {
                    html!{
                        <div class="text-center flex-grow text-red-400 table"><span class="table-cell align-middle">{t.t("calculation not possible", &state.language)}</span></div>
                    }
                }
            }
        </div>
    }
}
