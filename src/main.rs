use crate::common::prelude::*;
use crate::components::prelude::*;
use dioxus::logger::tracing::Level;
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t};
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};
use unic_langid::langid;

mod common;
mod components;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[redirect("/", || Route::AlcoholCalc {})]
    #[route("/alcohol")]
    AlcoholCalc{},
    // #[route("/ibu")]
    // IBUCalc{},
    // #[route("/color")]
    // ColorCalc{},
    // #[route("/color-conversion")]
    // ColorConversionCalc{},
    // #[route("/brewhouse")]
    // BrewhouseCalc{},
}

const FAVICON: Asset = asset!("/assets/images/favicon.ico");
const APP_CSS: Asset = asset!("/assets/css/app.css");

fn main() {
    dioxus::logger::init(Level::INFO).expect("unable to init logger");
    dioxus::launch(App);
}

fn i18n_config() -> I18nConfig {
    I18nConfig::new(langid!("en"))
        .with_locale((langid!("en"), include_str!("../i18n/en.ftl")))
        .with_locale((langid!("de"), include_str!("../i18n/de.ftl")))
        .with_locale((langid!("ru"), include_str!("../i18n/ru.ftl")))
}

#[component]
fn App() -> Element {
    let state =
        use_synced_storage::<LocalStorage, AppState>(STORAGE_KEY.to_string(), AppState::default);

    use_init_i18n(i18n_config);
    set_theme(&state.read().theme);

    rsx! {
        document::Title { {t!("title")} }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: APP_CSS }
        Router::<Route> {}
    }
}
