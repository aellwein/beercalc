use crate::common::prelude::*;
use crate::components::lang;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn AlcoholCalculator() -> Element {
    rsx! {
        Fragment {
            Header { active: Route::AlcoholCalculator }
            div { class: "text-2xl my-3", {t!("alcohol_calculator")} }
            GravityUnitPicker {}
            GravityPicker {}
        }
    }
}
