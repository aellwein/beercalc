//! Calculators for homebrewers.
//!
//! Contains different useful calculators:
//! * [`components::prelude::AlcoholCalculator`] - alcohol by volume/weight calculator.
//! * [`components::prelude::IBUCalculator`] - Bitterness (IBU) calculator.
//! * [`components::prelude::EbcCalculator`] - beer color (EBC) calculator.
//! * [`components::prelude::BrewhouseEfficiencyCalculator`] - brewhouse efficiency calculator.
//!
use crate::components::prelude::*;
use common::prelude::*;
use dioxus::logger::tracing::Level;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};
use unic_langid::langid;

mod common;
mod components;

const FAVICON: Asset = asset!("../assets/images/favicon.ico");
const APP_CSS: Asset = asset!("../assets/css/app.css");
const FONTS_CSS: Asset = asset!("../assets/css/fonts.css");
const _FONTS: Asset = asset!("../assets/fonts", AssetOptions::folder());

pub static STATE: GlobalSignal<CalcState> = Signal::global(CalcState::default);

fn i18n_config() -> I18nConfig {
    I18nConfig::new(langid!("en"))
        .with_locale((langid!("en"), include_str!("../assets/i18n/en.ftl")))
        .with_locale((langid!("de"), include_str!("../assets/i18n/de.ftl")))
        .with_locale((langid!("ru"), include_str!("../assets/i18n/ru.ftl")))
}

/// Entry point for the web application.
/// Spawns a future for the main async function to run and wait for it to finish.
fn main() {
    dioxus::logger::init(Level::INFO).expect("unable to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_init_i18n(i18n_config);

    // invalidate commit hash if it doesn't match in state
    if get_commit_hash() != STATE.read().commit_hash {
        let mut new_state = STATE.read().clone();
        new_state.commit_hash = get_commit_hash();
        *STATE.write() = new_state;
    }
    set_theme(&STATE.read().theme);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: APP_CSS }
        document::Link { rel: "stylesheet", href: FONTS_CSS }
        document::Title { {t!("title")} }

        div { class: "container p-2 dark:bg-gray-800 bg-neutral-50 mx-auto",
            div { class: "flex flex-col gap-1",
                Router::<Route> {}
                Footer {}
            }
        }
    }
}
