use crate::common::prelude::*;
use crate::components::prelude::*;
use enum_iterator::all;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yewdux::prelude::*;

#[function_component]
pub fn EbcCalculator() -> Html {
    let (state, dispatch) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    let units = [Gravity::Plato(0.0), Gravity::Brix(0.0)]
        .iter()
        .map(|g| {
            html! {
                <option value={g.translator_key()} key={g.translator_key()} selected={&state.chosen_unit == g}>{t.t(g.translator_key().as_str(), &state.language)}</option>
            }
        })
        .collect::<Vec<_>>();
    html! {
        <>
            <Header active={Route::BeerColorCalculator} />
            <div class="flex flex-col gap-4 dark:text-gray-400">
                <div class="text-2xl my-3">{t.t("color calculator", &state.language)}</div>
                <div class="flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4">
                     <div class="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t.t("original gravity", &state.language)}</div>
                        <input
                                type="number"
                                min=".1"
                                max="40"
                                step=".1"
                                class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                                value={format_gravity(&state.original_gravity)}
                                onchange={dispatch.reduce_callback_with(change_og_value)} />
                            <select class="p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid
                                    dark:bg-gray-700 dark:text-gray-300" onchange={dispatch.reduce_callback_with(change_og_unit)}>
                              {for units}
                            </select>
                     </div>
                     <div class="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t.t("boiling time", &state.language)}</div>
                        <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                            type="number"
                            min="1"
                            max="600"
                            step="1"
                            value={format!(r#"{}"#, state.ibu.boiling)}
                            onchange={dispatch.reduce_callback_with(change_boiling_time)}
                        />
                        <div>{t.t("minutes", &state.language)}</div>
                     </div>
                </div>
                <MaltAdder />
                <EbcColorDisplay />
            </div>
            <Footer />
        </>
    }
}

fn add_malt(s: Rc<CalcState>) -> Rc<CalcState> {
    let malt = Malt {
        amount: 5.0,
        mass_unit: MassUnit::Kilogram,
        color: 8.0,
    };
    let mut grain = s.grain.clone();
    grain.malt.push(malt);
    CalcState {
        grain,
        ..(*s).clone()
    }
    .into()
}

fn remove_grain(idx: usize) -> impl Fn(Rc<CalcState>) -> Rc<CalcState> {
    move |s: Rc<CalcState>| {
        let mut grain = s.grain.clone();
        grain.malt.remove(idx);
        CalcState {
            grain,
            ..(*s).clone()
        }
        .into()
    }
}

fn change_grain_amount(idx: usize) -> impl Fn(Rc<CalcState>, Event) -> Rc<CalcState> {
    move |s: Rc<CalcState>, e: Event| {
        let new_amount = e
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap_or(5.0);
        let mut grain = s.grain.clone();
        grain.malt[idx].amount = new_amount;
        CalcState {
            grain,
            ..(*s).clone()
        }
        .into()
    }
}

fn change_mass_unit(idx: usize) -> impl Fn(Rc<CalcState>, Event) -> Rc<CalcState> {
    move |s: Rc<CalcState>, e: Event| {
        let new_unit = e
            .target()
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap()
            .value();
        let mut grain = s.grain.clone();
        grain.malt[idx].mass_unit = new_unit.into();
        CalcState {
            grain,
            ..(*s).clone()
        }
        .into()
    }
}

fn change_grain_color(idx: usize) -> impl Fn(Rc<CalcState>, Event) -> Rc<CalcState> {
    move |s: Rc<CalcState>, e: Event| {
        let new_color = e
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap_or(5.0);
        let mut grain = s.grain.clone();
        grain.malt[idx].color = new_color;
        CalcState {
            grain,
            ..(*s).clone()
        }
        .into()
    }
}

fn grain_additions(s: Rc<CalcState>, dispatch: Dispatch<CalcState>, t: &Translator) -> Vec<VNode> {
    let mut result = vec![];
    for (idx, grain) in s.grain.malt.iter().enumerate() {
        let mass_units = all::<MassUnit>()
            .map(|u| {
                html! {
                    <option value={u.translator_key()} key={u.translator_key()} selected={grain.mass_unit == u}>{t.t(u.translator_key().as_str(), &s.language)}</option>
                }
            }).collect::<Vec<_>>();

        result.push(html! {
            <>
                <div class="flex flex-row gap-4 items-baseline flex-wrap">
                    <div>{format!(r#"{} #{}"#, t.t("malt", &s.language), idx + 1)}</div>
                    <button class="py-2 px-2 bg-red-300 dark:bg-red-900 dark:hover:bg-red-700 text-white hover:bg-red-600 p-1" onclick={dispatch.reduce_callback(remove_grain(idx))}>{"X"}</button>
                </div>
                <div class="flex flex-row items-baseline gap-4">
                    <div>{t.t("mass", &s.language)}</div>
                    <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        type="number" min="0" max="100000" step=".1" value={format!(r#"{:.1}"#, grain.amount)} onchange={dispatch.reduce_callback_with(change_grain_amount(idx))} />
                </div>
                <div class="flex flex-row items-baseline gap-4">
                    <div>{t.t("mass unit", &s.language)}</div>
                    <select class="p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300"
                         onchange={dispatch.reduce_callback_with(change_mass_unit(idx))}>
                        {for mass_units}
                    </select>
                </div>
                <div class="flex flex-row items-baseline gap-4">
                    <div>{t.t("color ebc", &s.language)}</div>
                    <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        type="number" min="1" max="10000" step="1" value={format!(r#"{:.0}"#, grain.color)} onchange={dispatch.reduce_callback_with(change_grain_color(idx))} />
                </div>
            </>
        });
    }
    result
}

#[function_component]
pub fn MaltAdder() -> Html {
    let t = use_context::<Translator>().unwrap();
    let (state, dispatch) = use_store::<CalcState>();
    let grain_additions = grain_additions(state.clone(), dispatch.clone(), &t);
    html! {
        <div class="flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4">
            {for grain_additions}
            <div class="mr-auto">
                <button class="py-2 px-4 bg-yellow-600 dark:bg-yellow-700 text-white dark:text-gray-100 rounded-lg
                    shadow-md dark:shadow-slate-600 hover:bg-yellow-800 dark:hover:bg-yellow-500 dark:hover:text-gray-800"
                    onclick={dispatch.reduce_callback(add_malt)}>
                    {t.t("add malt", &state.language)}
                </button>
            </div>
        </div>
    }
}

#[function_component]
pub fn EbcColorDisplay() -> Html {
    let (state, _dispatch) = use_store::<CalcState>();
    if state.grain.malt.is_empty() {
        return html! {
            <div/>
        };
    } else {
        let ebc = calculate_ebc(
            &state.original_gravity,
            &state.grain.malt,
            state.ibu.boiling,
        );
        let ebc_class_name = match ebc {
            0..100 => format!("ebc-{}", ebc),
            _ => "ebc-100".to_string(),
        };
        return html! {
            <div class="flex flex-row shadow-md dark:shadow-slate-600 p-4 gap-4 items-baseline">
                <div class="my-auto grow"><strong>{format!(r#"{} EBC"#, ebc)}</strong></div>
                <div class={ebc_class_name + " grow"} style="height: 100px"></div>
            </div>
        };
    }
}
