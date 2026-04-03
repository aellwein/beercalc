use crate::common::prelude::*;
use crate::components::lang;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn IbuCalculator() -> Element {
    let state =
        use_synced_storage::<LocalStorage, CalcState>(STATE_NAME.to_string(), CalcState::default);
    let language = use_signal(|| state.read().language);
    rsx! {
        div {
            Header { active: Route::IbuCalculator, language: language.clone() }
            div {}
        }
    }
}
