use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use yew_router::prelude::*;

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
    #[at("/brewhouse")]
    BrewhouseEfficiencyCalculator,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Sequence)]
pub enum Calculator {
    Alcohol,
    Ibu,
    EbcColor,
    BrewhouseEfficiency,
}
