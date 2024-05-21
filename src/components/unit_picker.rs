use crate::common::prelude::*;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;

fn change_to_brix(s: Rc<CalcState>) -> Rc<CalcState> {
    let chosen_unit = Gravity::Brix(0.0);
    let original_gravity = s.original_gravity.to_brix();
    let final_gravity = s.final_gravity.to_brix();
    CalcState {
        chosen_unit,
        original_gravity,
        final_gravity,
        ..(*s).clone()
    }
    .into()
}
fn change_to_plato(s: Rc<CalcState>) -> Rc<CalcState> {
    let chosen_unit = Gravity::Plato(0.0);
    let original_gravity = s.original_gravity.to_plato();
    let final_gravity = s.final_gravity.to_plato();
    CalcState {
        chosen_unit,
        original_gravity,
        final_gravity,
        ..(*s).clone()
    }
    .into()
}

#[function_component]
pub fn UnitPicker() -> Html {
    let (state, dispatch) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    let lang = use_store::<CalcState>().0.language.clone();
    let to_brix = dispatch.reduce_callback(|s| change_to_brix(s));
    let to_plato = dispatch.reduce_callback(|s| change_to_plato(s));
    html! {
        <div class="flex flex-row gap-4 shadow-md p-4">
            <span>{t.t("gravity unit", &lang)}</span>
            <div>
            <input type="radio" class="dark:bg-gray-700 dark:text-gray-300" id="brix" name="unit" checked={matches!(state.chosen_unit, Gravity::Brix(_))} onchange={to_brix} />
                <label for="brix">{format!(" {}", t.t("brix", &lang))}</label>
            </div>
            <div>
            <input type="radio" class="dark:bg-gray-700 dark:text-gray-300" id="plato" name="unit" checked={matches!(state.chosen_unit, Gravity::Plato(_))} onchange={to_plato} />
                <label for="plato">{format!(" {}", t.t("plato", &lang))}</label>
            </div>
        </div>
    }
}
