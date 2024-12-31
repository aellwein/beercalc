use crate::common::prelude::*;
use enum_iterator::all;
use std::{mem::replace, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{prelude::*, virtual_dom::VNode};
use yewdux::prelude::*;

fn add_hops(s: Rc<CalcState>) -> Rc<CalcState> {
    let hops_addition = Hops {
        form: HopsForm::Pellets,
        amount: 5.0,
        alpha: 7.0,
        boil_time: 90.0,
        ibu: 0.0,
    };
    let mut ibu = s.ibu.clone();
    let mut hops = s.ibu.hops.clone();
    hops.push(hops_addition);
    ibu.hops = hops;
    CalcState {
        ibu,
        ..(*s).clone()
    }
    .into()
}

fn remove_hops(idx: i32) -> impl Fn(Rc<CalcState>) -> Rc<CalcState> {
    move |s: Rc<CalcState>| {
        let mut ibu = s.ibu.clone();
        let mut hops = s.ibu.hops.clone();
        hops.remove(idx as usize);
        ibu.hops = hops;
        CalcState {
            ibu,
            ..(*s).clone()
        }
        .into()
    }
}

fn change_hops_form(idx: i32) -> impl Fn(Rc<CalcState>, Event) -> Rc<CalcState> {
    move |s: Rc<CalcState>, e: Event| {
        let new_form = e
            .target()
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap()
            .value();
        let mut ibu = s.ibu.clone();
        let mut hops = s.ibu.hops.clone();
        hops[idx as usize].form = new_form.into();
        ibu.hops = hops;
        CalcState {
            ibu,
            ..(*s).clone()
        }
        .into()
    }
}

fn change_hops_amount(idx: i32) -> impl Fn(Rc<CalcState>, Event) -> Rc<CalcState> {
    move |s: Rc<CalcState>, e: Event| {
        let new_amount = e
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap_or(5.0);

        let mut ibu = s.ibu.clone();
        let mut hops = s.ibu.hops.clone();
        hops[idx as usize].amount = new_amount;
        ibu.hops = hops;
        CalcState {
            ibu,
            ..(*s).clone()
        }
        .into()
    }
}

fn change_alpha(idx: i32) -> impl Fn(Rc<CalcState>, Event) -> Rc<CalcState> {
    move |s: Rc<CalcState>, e: Event| {
        let new_alpha = e
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap_or(7.0);

        let mut ibu = s.ibu.clone();
        let mut hops = s.ibu.hops.clone();
        hops[idx as usize].alpha = new_alpha;
        ibu.hops = hops;
        CalcState {
            ibu,
            ..(*s).clone()
        }
        .into()
    }
}

fn change_boil_time(idx: i32) -> impl Fn(Rc<CalcState>, Event) -> Rc<CalcState> {
    move |s: Rc<CalcState>, e: Event| {
        let new_boil = e
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap_or(90.0)
            .min(s.ibu.boiling);

        let mut ibu = s.ibu.clone();
        let mut hops = s.ibu.hops.clone();
        hops[idx as usize].boil_time = new_boil;
        ibu.hops = hops;
        CalcState {
            ibu,
            ..(*s).clone()
        }
        .into()
    }
}

fn hops_additions(s: Rc<CalcState>, dispatch: Dispatch<CalcState>, t: &Translator) -> Vec<VNode> {
    let mut result = vec![];
    let mut key = -1;
    for hops in s.ibu.hops.iter() {
        key += 1;
        let forms = all::<HopsForm>()
        .map(|f| {
            html! {
                <option value={f.translator_key()} key={f.translator_key()} selected={hops.form == f}>{t.t(f.translator_key().as_str(), &s.language)}</option>
            }
        })
        .collect::<Vec<_>>();
        result.push(
            html! {
            <>
                <div class="flex flex-row items-baseline gap-4">
                    <div class="underline text-right">{t.t("hops addition", &s.language)}{format!(r#" #{}"#, key+1)}</div>
                    <button class="py-2 px-2 bg-red-300 dark:bg-red-900 dark:hover:bg-red-700 text-white hover:bg-red-600 p-1" onclick={dispatch.reduce_callback(remove_hops(key))}>{"X"}</button>
                </div>
                <div class="flex flex-row items-baseline gap-4">
                    <div class="">{t.t("form", &s.language)}</div>
                    <select class="p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid dark:bg-gray-700 dark:text-gray-300" onchange={dispatch.reduce_callback_with(change_hops_form(key))}>
                        {for forms}
                    </select>
                </div>
                <div class="flex flex-row items-baseline gap-4">
                    <div>{t.t("amount", &s.language)}</div>
                    <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        type="number" min="0" max="5000" step=".1" value={format!(r#"{:.1}"#,hops.amount)} onchange={dispatch.reduce_callback_with(change_hops_amount(key))} />
              </div>
              <div class="flex flex-row items-baseline gap-4">
                    <div>{t.t("alpha acid", &s.language)}</div>
                    <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700
                        dark:text-gray-300" type="number" min="0" max="90" step=".1" value={format!(r#"{:.1}"#,hops.alpha)} onchange={dispatch.reduce_callback_with(change_alpha(key))} />
              </div>
              <div class="flex flex-row items-baseline gap-4">
                    <div>{t.t("boil time", &s.language)}</div>
                    <input class="border-gray-300 p-1 border-solid border focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        type="number" min="0" max={format!(r#"{:.0}"#, s.ibu.boiling)} value={format!(r#"{:.0}"#, hops.boil_time)} onchange={dispatch.reduce_callback_with(change_boil_time(key))} />
              </div>
              <div class="flex flex-row items-baseline gap-4">
                    <div class="grow">{t.t("bitterness", &s.language)}</div>
                    <div class="text-right dark:text-gray-200">{format!(r#"{:.1}"#, hops.ibu)}{" IBU"}</div>
              </div>
            </>
        });
    }
    result
}

fn recalc_ibu(s: Rc<CalcState>, dispatch: Dispatch<CalcState>) {
    let iso_speed_factor = 0.046 * (0.031 * s.ibu.flameout_temp).exp();
    let add_iso_time = iso_speed_factor * s.ibu.flameout;

    for (idx, hops) in s.ibu.hops.iter().enumerate() {
        let hops_factor = match hops.form {
            HopsForm::Whole => 1.0,
            HopsForm::Pellets => 1.1,
        };

        let new_ibu = hops_factor
            * ((hops.amount * hops.alpha * 10.0 / s.ibu.volume)
                * (1.65 * 0.000125_f64.powf(0.004 * s.original_gravity.to_plato().value()))
                * (1.0 - (-0.04 * (hops.boil_time + add_iso_time)).exp())
                / 4.15);

        if hops.ibu != new_ibu {
            let mut ibu = s.ibu.clone();
            let mut hops_new = s.ibu.hops.clone();
            let mut new_item = hops.clone();
            new_item.ibu = new_ibu;
            let _ = replace(&mut hops_new[idx], new_item);
            ibu.hops = hops_new;
            dispatch.set(CalcState {
                ibu,
                ..(*s).clone()
            });
        }
    }
}

fn sum_ibu(s: Rc<CalcState>) -> f64 {
    s.ibu.hops.iter().map(|h| h.ibu).sum()
}

#[function_component]
pub fn HopsAdder() -> Html {
    let (state, dispatch) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    recalc_ibu(state.clone(), dispatch.clone());
    let ha = hops_additions(state.clone(), dispatch.clone(), &t);
    html! {
        <div class="flex flex-col gap-4 p-4 shadow-md dark:shadow-slate-600">
            {for ha}
            <div class="flex flex-row gap-4">
                <div class="grow dark:text-gray-200"><strong>{t.t("total bitterness", &state.language)}</strong></div>
                <div class="text-right dark:text-gray-200"><strong>{format!(r#"{:.1} IBU"#, sum_ibu(state.clone()))}</strong></div>
            </div>
            <div class="mr-auto">
                <button class="py-2 px-4 text-white dark:text-gray-200 bg-green-500 dark:bg-green-900 rounded-lg shadow-md dark:shadow-slate-600
                hover:bg-green-700 dark:hover:bg-green-800" onclick={dispatch.reduce_callback(add_hops)}>{t.t("add hops", &state.language)}</button>
          </div>
        </div>
    }
}
