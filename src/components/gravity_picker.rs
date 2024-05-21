use crate::common::prelude::*;
use crate::components::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

fn change_og(s: Rc<CalcState>, e: Event) -> Rc<CalcState> {
    let new_og = e
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>();

    let original_gravity = match new_og {
        Ok(v) => s.original_gravity.update_value(v),
        Err(_) => Gravity::Brix(12.0),
    };
    CalcState {
        original_gravity,
        ..(*s).clone()
    }
    .into()
}

fn change_fg(s: Rc<CalcState>, e: Event) -> Rc<CalcState> {
    let new_fg = e
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>();

    let final_gravity = match new_fg {
        Ok(v) => s.final_gravity.update_value(v),
        Err(_) => Gravity::Brix(6.0),
    };
    CalcState {
        final_gravity,
        ..(*s).clone()
    }
    .into()
}

fn format_gravity(g: &Gravity) -> String {
    match g {
        Gravity::Plato(v) => format!("{:.1}", v),
        Gravity::Brix(v) => format!("{:.1}", v),
        Gravity::Oechsle(v) => format!("{:.1}", v),
        Gravity::SG(v) => format!("{:.3}", v),
    }
}

#[function_component]
pub fn GravityPicker() -> Html {
    let t = use_context::<Translator>().unwrap();
    let (state, dispatch) = use_store::<CalcState>();
    let original_gravity = format_gravity(&state.original_gravity);
    let final_gravity = format_gravity(&state.final_gravity);

    let change_og = dispatch.reduce_callback_with(|s, e| change_og(s, e));
    let change_fg = dispatch.reduce_callback_with(|s, e| change_fg(s, e));

    html! {
        <div class="flex flex-col gap-4 shadow-md p-4 items-baseline">

            <div class="flex flex-row gap-4 items-baseline flex-wrap">
                <span>{t.t("original gravity", &state.language)}</span>

                <input class="border-gray-300 p-1 border-solid border focus:border-blue-300
                focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                type="number" id="og" min=".1" max="60.0" step=".1" value={original_gravity} onchange={change_og} />

                <ShowUnits gravity={state.original_gravity.clone()} />
            </div>

            <div class="flex flex-row gap-4 items-baseline flex-wrap">
                <span>{t.t("final gravity", &state.language)}</span>

                <input class="border-gray-300 p-1 border-solid border focus:border-blue-300
                focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                type="number" id="fg" min=".1" max="60.0" step=".1" value={final_gravity} onchange={change_fg} />

                <ShowUnits gravity={state.final_gravity.clone()} />
            </div>

        </div>
    }
}
