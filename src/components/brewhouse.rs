use crate::{common::prelude::*, components::header::Header};
use yew::prelude::*;

#[function_component]
pub fn BrewhouseEfficiencyCalculator() -> Html {
    html! {
        <Header active={Route::BrewhouseEfficiencyCalculator} />
    }
}
