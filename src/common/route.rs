//! This module defines the routes for the application.
use crate::components::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    // Placeholder component, will never be rendered due to the redirect in the Route enum.
    todo!()
}

#[component]
fn NotFound(_not_found: Vec<String>) -> Element {
    rsx!(
        div { "404 - Not Found" }
    )
}

/// Navigation routes for the application.
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    #[redirect("/", || Route::AlcoholCalculator)]
    Home {},
    #[route("/alcohol")]
    AlcoholCalculator,
    #[route("/ibu")]
    IbuCalculator,
    #[route("/color")]
    BeerColorCalculator,
    #[route("/color-conversion")]
    ColorConversionCalculator,
    #[route("/brewhouse")]
    BrewhouseEfficiencyCalculator,
    #[route("/:.._not_found")]
    NotFound { _not_found: Vec<String> },
}

// Tuple of all calculators with their corresponding route and i18n key for the header.
pub const CALCULATORS: [(Route, &str); 5] = [
    (Route::AlcoholCalculator, "alcohol_calculator"),
    (Route::IbuCalculator, "ibu_calculator"),
    (Route::BeerColorCalculator, "color_calculator"),
    (Route::ColorConversionCalculator, "color_conversion"),
    (Route::BrewhouseEfficiencyCalculator, "brewhouse_calculator"),
];
