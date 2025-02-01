//! This module defines the routes for the application.
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use yew_router::prelude::*;

/// Navigation routes for the application.
#[derive(Clone, Routable, PartialEq, Serialize, Deserialize, Sequence)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/alcohol")]
    AlcoholCalculator,
    #[at("/ibu")]
    IbuCalculator,
    #[at("/color")]
    BeerColorCalculator,
    #[at("/color-conversion")]
    ColorConversionCalculator,
    #[at("/brewhouse")]
    BrewhouseEfficiencyCalculator,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Enum for different calculators, used to determine which calculator to display.
#[derive(Sequence)]
pub enum Calculator {
    Alcohol,
    Ibu,
    EbcColor,
    ColorConversion,
    BrewhouseEfficiency,
}
