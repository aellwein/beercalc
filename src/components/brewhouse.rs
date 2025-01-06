use crate::common::prelude::*;
use crate::components::prelude::*;
use enum_iterator::all;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;

fn change_grain_amount(state: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_amount = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(5.0);

    let mut grain = state.brewhouse.clone();
    grain.grain_mass = new_amount;
    CalcState {
        brewhouse: grain,
        ..(*state).clone()
    }
    .into()
}

fn change_grain_unit(state: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_unit = event
        .target()
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap()
        .value();
    let mut brewhouse = state.brewhouse.clone();
    brewhouse.grain_mass_unit = new_unit.into();
    CalcState {
        brewhouse,
        ..(*state).clone()
    }
    .into()
}

fn change_volume_measured_at(vma: VolumeMeasuredAt) -> impl Fn(Rc<CalcState>) -> Rc<CalcState> {
    move |state: Rc<CalcState>| {
        let mut brewhouse = state.brewhouse.clone();
        brewhouse.volume_measured_at = vma.clone();
        CalcState {
            brewhouse,
            ..(*state).clone()
        }
        .into()
    }
}

fn change_volume_measured_at2(vma: VolumeMeasuredAt) -> impl Fn(Rc<CalcState>) -> Rc<CalcState> {
    move |state: Rc<CalcState>| {
        let mut brewhouse = state.brewhouse.clone();
        brewhouse.grain_mass_from_brewhouse.volume_measured_at = vma.clone();
        CalcState {
            brewhouse,
            ..(*state).clone()
        }
        .into()
    }
}

fn change_wort_volume(state: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_volume = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(28.0);

    let mut brewhouse = state.brewhouse.clone();
    brewhouse.wort_volume = new_volume;
    CalcState {
        brewhouse,
        ..(*state).clone()
    }
    .into()
}

fn change_brewhouse_efficiency(state: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_efficiency = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(65.6);

    let mut brewhouse = state.brewhouse.clone();
    brewhouse.grain_mass_from_brewhouse.brewhouse_efficiency = new_efficiency;
    CalcState {
        brewhouse,
        ..(*state).clone()
    }
    .into()
}

fn change_wort_volume2(state: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_volume = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(28.0);

    let mut brewhouse = state.brewhouse.clone();
    brewhouse.grain_mass_from_brewhouse.wort_volume = new_volume;
    CalcState {
        brewhouse,
        ..(*state).clone()
    }
    .into()
}

fn change_og_value2(state: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_value = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(12.0);

    let mut brewhouse = state.brewhouse.clone();
    brewhouse.grain_mass_from_brewhouse.original_gravity = brewhouse
        .grain_mass_from_brewhouse
        .original_gravity
        .update_value(new_value);
    CalcState {
        brewhouse,
        ..(*state).clone()
    }
    .into()
}

pub fn change_og_unit2(s: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_unit = event
        .target()
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap()
        .value();

    let new_og = match new_unit.as_str() {
        "plato" => s
            .brewhouse
            .grain_mass_from_brewhouse
            .original_gravity
            .to_plato(),
        "brix" => s
            .brewhouse
            .grain_mass_from_brewhouse
            .original_gravity
            .to_brix(),
        _ => s
            .brewhouse
            .grain_mass_from_brewhouse
            .original_gravity
            .clone(),
    };

    let mut brewhouse = s.brewhouse.clone();
    brewhouse.grain_mass_from_brewhouse.original_gravity = new_og;

    CalcState {
        brewhouse,
        ..(*s).clone()
    }
    .into()
}

#[function_component]
pub fn BrewhouseEfficiencyCalculator() -> Html {
    let (state, dispatch) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    let mass_units = all::<MassUnit>()
            .map(|u| {
                html! {
                    <option value={u.translator_key()} key={u.translator_key()} selected={state.brewhouse.grain_mass_unit == u}>{t.t(u.translator_key().as_str(), &state.language)}</option>
                }
            }).collect::<Vec<_>>();
    let units = [Gravity::Plato(0.0), Gravity::Brix(0.0)]
                .iter()
                .map(|g| {
                    html! {
                        <option value={g.translator_key()} key={g.translator_key()} selected={&state.chosen_unit == g}>{t.t(g.translator_key().as_str(), &state.language)}</option>
                    }
                })
                .collect::<Vec<_>>();
    let units2 = [Gravity::Plato(0.0), Gravity::Brix(0.0)]
            .iter()
            .map(|g| {
                html! {
                    <option value={g.translator_key()} key={g.translator_key()} selected={&state.brewhouse.grain_mass_from_brewhouse.original_gravity == g}>{t.t(g.translator_key().as_str(), &state.language)}</option>
                }
            })
            .collect::<Vec<_>>();

    html! {
        <>
            <Header active={Route::BrewhouseEfficiencyCalculator} />
            <div class="flex flex-col gap-4 dark:text-gray-400">
                <div class="text-2xl my-3">{t.t("brewhouse calculator", &state.language)}</div>
                <div class="flex flex-row gap-4 flex-wrap">


                    <div class="flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4 items-baseline grow">
                        <div class="text-xl">{t.t("brewhouse efficiency", &state.language)}</div>
                        <div class="flex flex-row gap-4 items-baseline flex-wrap">
                            <div>{t.t("grain mass", &state.language)}</div>
                            <input type="number" min=".1" max="100000" step=".1"
                                class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                                value={format!(r#"{:.1}"#, state.brewhouse.grain_mass)}
                                onchange={dispatch.reduce_callback_with(change_grain_amount)}
                            />
                            <select class="p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300" onchange={dispatch.reduce_callback_with(change_grain_unit)}>
                            {for mass_units}
                            </select>
                        </div>
                        <div class="flex flex-row gap-4 items-baseline flex-wrap">
                            <div>{t.t("original gravity", &state.language)}</div>
                            <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                                    type="number"
                                    min=".1"
                                    max="60"
                                    step=".1"
                                    value={format_gravity(&state.original_gravity)}
                                    onchange={dispatch.reduce_callback_with(change_og_value)}
                            />
                            <select class="p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300"
                                onchange={dispatch.reduce_callback_with(change_og_unit)}>
                                {for units}
                            </select>
                        </div>
                        <div class="flex flex-row gap-4 items-baseline flex-wrap">
                            <div>{t.t("wort volume", &state.language)}</div>
                            <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                                type="number"
                                min=".1"
                                max="1000"
                                step=".1"
                                value={format!(r#"{:.1}"#, state.brewhouse.wort_volume)}
                                onchange={dispatch.reduce_callback_with(change_wort_volume)}
                            />
                            <div>{t.t("liter", &state.language)}</div>
                        </div>
                        <div class="flex flex-row gap-4 items-baseline flex-wrap">
                            <div>{t.t("volume measured at", &state.language)}</div>
                            <label>
                                <input type="radio" class="dark:bg-gray-700 dark:text-gray-300" id="100_C" name="vma" checked={state.brewhouse.volume_measured_at == VolumeMeasuredAt::HundredDegreesCelsius}
                                    onchange={dispatch.reduce_callback(change_volume_measured_at(VolumeMeasuredAt::HundredDegreesCelsius))}
                                />
                                {"  100 °C"}
                            </label>
                            <label>
                                <input type="radio" class="dark:bg-gray-700 dark:text-gray-300" id="20_C" name="vma" checked={state.brewhouse.volume_measured_at == VolumeMeasuredAt::TwentyDegreesCelsius}
                                    onchange={dispatch.reduce_callback(change_volume_measured_at(VolumeMeasuredAt::TwentyDegreesCelsius))}
                                />
                                {"  20 °C"}
                            </label>
                        </div>
                        <div><strong>{format!(r#"{}: {:.1} %"#, t.t("brewhouse efficiency", &state.language), calculate_brewhouse_efficiency(&state.original_gravity, &state.brewhouse))}</strong></div>
                    </div>



                    <div class="flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4 items-baseline grow">
                        <div class="text-xl">{t.t("grain mass relative to brewhouse efficiency", &state.language)}</div>
                        <div class="flex flex-row gap-4 items-baseline flex-wrap">
                            <div>{t.t("brewhouse efficiency", &state.language)}</div>
                            <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                                type="number"
                                min=".1"
                                max="100"
                                step=".1"
                                value={format!(r#"{:.1}"#, state.brewhouse.grain_mass_from_brewhouse.brewhouse_efficiency)}
                                onchange={dispatch.reduce_callback_with(change_brewhouse_efficiency)}
                            />{" %"}
                        </div>
                        <div class="flex flex-row gap-4 items-baseline flex-wrap">
                            <div>{t.t("original gravity", &state.language)}</div>
                            <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                                type="number"
                                min=".1"
                                max="60"
                                step=".1"
                                value={format_gravity(&state.brewhouse.grain_mass_from_brewhouse.original_gravity)}
                                onchange={dispatch.reduce_callback_with(change_og_value2)}
                            />
                            <select class="p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300"
                                onchange={dispatch.reduce_callback_with(change_og_unit2)}>
                                {for units2}
                            </select>
                        </div>
                        <div class="flex flex-row gap-4 items-baseline flex-wrap">
                            <div>{t.t("wort volume", &state.language)}</div>
                            <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                                type="number"
                                min=".1"
                                max="1000"
                                step=".1"
                                value={format!(r#"{:.1}"#, state.brewhouse.grain_mass_from_brewhouse.wort_volume)}
                                onchange={dispatch.reduce_callback_with(change_wort_volume2)}
                            />
                            <div>{t.t("liter", &state.language)}</div>
                        </div>
                        <div class="flex flex-row gap-4 items-baseline flex-wrap">
                            <div>{t.t("volume measured at", &state.language)}</div>
                            <label>
                                <input type="radio" class="dark:bg-gray-700 dark:text-gray-300" id="gm_100_C" name="vma2" checked={state.brewhouse.grain_mass_from_brewhouse.volume_measured_at == VolumeMeasuredAt::HundredDegreesCelsius}
                                    onchange={dispatch.reduce_callback(change_volume_measured_at2(VolumeMeasuredAt::HundredDegreesCelsius))}
                                />
                                {"  100 °C"}
                            </label>
                            <label>
                                <input type="radio" class="dark:bg-gray-700 dark:text-gray-300" id="gm_20_C" name="vma2" checked={state.brewhouse.grain_mass_from_brewhouse.volume_measured_at == VolumeMeasuredAt::TwentyDegreesCelsius}
                                    onchange={dispatch.reduce_callback(change_volume_measured_at2(VolumeMeasuredAt::TwentyDegreesCelsius))}
                                />
                                {"  20 °C"}
                            </label>
                        </div>
                        <div><strong>{format!(r#"{}: {:.1} {}"#, t.t("grain mass", &state.language), calculate_grain_mass_from_brewhouse(&state.brewhouse.grain_mass_from_brewhouse), t.t("kg", &state.language))}</strong></div>
                    </div>
                </div>
            </div>

            <Footer />
        </>
    }
}
