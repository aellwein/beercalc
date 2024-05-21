use crate::common::prelude::*;
use crate::components::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn AlcoholCalculator() -> Html {
    let (state, _) = use_store::<CalcState>();
    let t = use_context::<Translator>().unwrap();
    html! {
        <>
            <Header active={Route::AlcoholCalculator} />
            <div class="flex flex-col gap-4 dark:text-gray-400">
                <div class="text-2xl my-3">{t.t("alcohol calculator", &state.language)}</div>
                <UnitPicker />
                <GravityPicker />
            </div>
            <Footer />
        </>
    }
}
