use crate::common::prelude::*;
use crate::components::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;

pub fn change_og_unit(s: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_unit = event
        .target()
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap()
        .value();

    let (chosen_unit, original_gravity, final_gravity) = match new_unit.as_str() {
        "plato" => (
            Gravity::Plato(0.0),
            s.original_gravity.to_plato(),
            s.final_gravity.to_plato(),
        ),
        "brix" => (
            Gravity::Brix(0.0),
            s.original_gravity.to_brix(),
            s.final_gravity.to_brix(),
        ),
        _ => (
            Gravity::Brix(0.0),
            s.original_gravity.clone(),
            s.final_gravity.clone(),
        ),
    };

    CalcState {
        chosen_unit,
        original_gravity,
        final_gravity,
        ..(*s).clone()
    }
    .into()
}

pub fn change_boiling_time(s: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_boiling_time = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(90.0);

    let ibu = Ibu {
        boiling: new_boiling_time,
        ..s.ibu.clone()
    };

    CalcState {
        ibu,
        ..(*s).clone()
    }
    .into()
}

fn change_volume(s: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_volume = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(50.0);

    let ibu = Ibu {
        volume: new_volume,
        ..s.ibu.clone()
    };

    CalcState {
        ibu,
        ..(*s).clone()
    }
    .into()
}

fn change_flameout_temp(s: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_temp = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(85.0);

    let ibu = Ibu {
        flameout_temp: new_temp,
        ..s.ibu.clone()
    };

    CalcState {
        ibu,
        ..(*s).clone()
    }
    .into()
}

fn change_flameout_time(s: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_flameout = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(5.0);

    let ibu = Ibu {
        flameout: new_flameout,
        ..s.ibu.clone()
    };

    CalcState {
        ibu,
        ..(*s).clone()
    }
    .into()
}

#[function_component]
pub fn IBUPreset() -> Html {
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
        <div class="flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4">
            <div class="flex flex-row gap-4 items-baseline flex-wrap">
                <div>{t.t("original gravity", &state.language)}</div>
                <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                type="number"
                min=".1"
                max="60.0"
                step=".1"
                value={format_gravity(&state.original_gravity)}
                onchange={dispatch.reduce_callback_with(change_og_value)} />

                <select class="p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300"
                  onchange={dispatch.reduce_callback_with(change_og_unit)}>
                {for units}
                </select>
            </div>
            <div class="flex flex-row gap-4 items-baseline flex-wrap">
                <div>{t.t("boiling time", &state.language)}</div>
                <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                    type="number" min="1" max="600" step="1" value={format!(r#"{}"#, state.ibu.boiling)} onchange={dispatch.reduce_callback_with(change_boiling_time)} />
                <div>{t.t("minutes", &state.language)}</div>
            </div>
            <div class="flex flex-row gap-4 items-baseline flex-wrap">
                <div>{t.t("volume", &state.language)}</div>
                <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                type="number" min="1" max="500" step="1" value={format!(r#"{}"#, state.ibu.volume)} onchange={dispatch.reduce_callback_with(change_volume)} />
                <div>{t.t("liter", &state.language)}</div>
            </div>
            <div class="flex flex-row gap-4 items-baseline flex-wrap">
                <div>{t.t("flameout time", &state.language)}</div>
                <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                type="number" min="0" max="90" step="1" value={format!(r#"{}"#, state.ibu.flameout)} onchange={dispatch.reduce_callback_with(change_flameout_time)} />
                <div>{t.t("minutes", &state.language)}</div>
            </div>
            <div class="flex flex-row gap-4 items-baseline flex-wrap">
                <div>{t.t("flameout temp", &state.language)}</div>
                <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                type="number" min="0" max="90" step="1" value={format!(r#"{}"#, state.ibu.flameout_temp)} onchange={dispatch.reduce_callback_with(change_flameout_temp)} />
                <div>{"°C"}</div>
            </div>
        </div>
    }
}
