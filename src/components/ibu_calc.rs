use crate::{common::prelude::*, components::header::Header};
use yew::prelude::*;

#[function_component]
pub fn IBUCalculator() -> Html {
    html! {
        <Header active={Route::IbuCalculator} />
    }
}
