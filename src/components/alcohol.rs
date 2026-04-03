use crate::common::prelude::*;
use crate::components::lang;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn AlcoholCalculator() -> Element {
    let state =
        use_synced_storage::<LocalStorage, CalcState>(STATE_NAME.to_string(), CalcState::default);
    let language = use_signal(|| state.read().language);
    rsx! {
        Fragment {
            Header { active: Route::AlcoholCalculator, language: language.clone() }
            div { class: "text-2xl my-3", {t!("alcohol_calculator")} }
            GravityUnitPicker { current_unit: state.read().chosen_unit.clone() }
        }
    }
}
