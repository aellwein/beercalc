use crate::common::prelude::*;
use crate::components::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yewdux::prelude::*;

fn show_color_units(color: &ColorUnit, t: Translator, lang: &Language) -> Html {
    let label = match color {
        ColorUnit::Ebc(_) => format!(
            r#"{:.1} {} / {:.1} {}"#,
            color.to_srm().value(),
            t.t(&color.to_srm().translator_key(), lang),
            color.to_lovibond().value(),
            t.t(&color.to_lovibond().translator_key(), lang),
        ),
        ColorUnit::Srm(_) => format!(
            r#"{:.1} {} / {:.1} {}"#,
            color.to_ebc().value(),
            t.t(&color.to_ebc().translator_key(), lang),
            color.to_lovibond().value(),
            t.t(&color.to_lovibond().translator_key(), lang),
        ),
        ColorUnit::Lovibond(_) => format!(
            r#"{:.1} {} / {:.1} {}"#,
            color.to_ebc().value(),
            t.t(&color.to_ebc().translator_key(), lang),
            color.to_srm().value(),
            t.t(&color.to_srm().translator_key(), lang),
        ),
    };
    html! {
        <span>
        {label}
        </span>
    }
}

fn change_color_value(state: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_color = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .parse::<f64>()
        .unwrap_or(12.0);

    let color = state.color_conversion.color.update_value(new_color);
    CalcState {
        color_conversion: ColorConversion { color },
        ..(*state).clone()
    }
    .into()
}

fn change_color_unit(state: Rc<CalcState>, event: Event) -> Rc<CalcState> {
    let new_unit = event
        .target()
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap()
        .value();

    let color = match new_unit.as_str() {
        "ebc" => ColorUnit::Ebc(state.color_conversion.color.value()),
        "srm" => ColorUnit::Srm(state.color_conversion.color.value()),
        "lovibond" => ColorUnit::Lovibond(state.color_conversion.color.value()),
        _ => state.color_conversion.color.clone(),
    };
    CalcState {
        color_conversion: ColorConversion { color },
        ..(*state).clone()
    }
    .into()
}

#[function_component]
pub fn ColorConversionCalculator() -> Html {
    let (state, dispatch) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    let color_units = [ColorUnit::Ebc(0.0), ColorUnit::Srm(0.0), ColorUnit::Lovibond(0.0)]
        .iter()
        .map(|c| {
            html! {
                <option value={c.translator_key()} key={c.translator_key()} selected={c.match_unit(&state.color_conversion.color)}>{t.t(c.translator_key().as_str(), &state.language)}</option>
            }
        })
        .collect::<Vec<_>>();
    html! {
        <>
            <Header active={Route::ColorConversionCalculator} />
            <div class="flex flex-col gap-4 dark:text-gray-400">
                <div class="text-2xl my-3">{t.t("color conversion", &state.language)}</div>
                    <div class="flex flex-row gap-4 shadow-md dark:shadow-slate-600 p-4 items-baseline">
                        <span>{t.t("malt color", &state.language)}</span>
                        <input
                            type="number" min=".1" max="200" step=".1"
                            value={format!("{:.1}", state.color_conversion.color.value())}
                            onchange={dispatch.reduce_callback_with(change_color_value)}
                            class="border-gray-300 p-1 border-solid border
                            focus:border-blue-300 focus:ring outline-none
                            dark:bg-gray-700 dark:text-gray-300" />
                        <select
                            class="p-1 appearance-none rounded-none bg-white border-gray-300 border border-solid
                                dark:bg-gray-700 dark:text-gray-300"
                                onchange={dispatch.reduce_callback_with(change_color_unit)}
                        >{for color_units}</select>
                        {show_color_units(&state.color_conversion.color, t.clone(), &state.language)}
                    </div>
            </div>
            <Footer />
        </>
    }
}
