use crate::common::prelude::*;
use crate::components::prelude::*;
use std::sync::Arc;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

pub const BODY_CLASSES: &str = "dark:bg-gray-800";

#[derive(Properties, Clone, PartialEq)]
pub struct AppProps {
    pub translator: Arc<Translator>,
}

#[function_component]
pub fn App(props: &AppProps) -> Html {
    let (state, _) = use_store::<CalcState>();
    set_page_title(props.translator.t("title", &state.language).as_str());
    set_body_classes(BODY_CLASSES);

    html! {
        <BrowserRouter>
            <ContextProvider<Translator> context={props.translator.as_ref().clone()}>
                <div class="container p-3 dark:bg-gray-800 mx-auto">
                    <Switch<Route> render={switch} />
                </div>
            </ContextProvider<Translator>>
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Redirect<Route> to={Route::AlcoholCalculator} /> },
        Route::AlcoholCalculator => html! {<AlcoholCalculator /> },
        Route::IbuCalculator => html! {<IBUCalculator />},
        Route::BeerColorCalculator => html! {<EbcCalculator />},
        Route::BrewhouseEfficiencyCalculator => html! {<BrewhouseEfficiencyCalculator />},
        Route::NotFound => html! {<NotFound />},
    }
}
