//! Calculators for homebrewers.
//!
//! Contains different useful calculators:
//! * [`components::prelude::AlcoholCalculator`] - alcohol by volume/weight calculator.
//! * [`components::prelude::IBUCalculator`] - Bitterness (IBU) calculator.
//! * [`components::prelude::EbcCalculator`] - beer color (EBC) calculator.
//! * [`components::prelude::BrewhouseEfficiencyCalculator`] - brewhouse efficiency calculator.
//!
use common::prelude::*;
use components::prelude::*;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

mod common;
mod components;

/// Main (async) entry point for the web application.
/// Sets up the error handler, page title, body classes, and the app props,
/// then renders the main [`App`] component.
async fn async_main() {
    console_error_panic_hook::set_once();

    let translator = Translator::new()
        .await
        .expect("unable to load translations");
    let props = AppProps {
        translator: Arc::new(translator),
    };

    yew::Renderer::<App>::with_props(props).render();
}

/// Entry point for the web application.
/// Spawns a future for the main async function to run and wait for it to finish.
fn main() {
    spawn_local(async_main());
}
