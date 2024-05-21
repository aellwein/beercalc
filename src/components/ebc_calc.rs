use crate::{common::prelude::*, components::header::Header};
use yew::prelude::*;

#[function_component]
pub fn EbcCalculator() -> Html {
    html! {
        <Header active={Route::BeerColorCalculator} />
    }
}
