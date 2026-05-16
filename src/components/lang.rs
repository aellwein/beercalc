use crate::{common::prelude::*, STATE};
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[component]
pub fn LanguageSwitcher() -> Element {
    let language = STATE.read().language;
    let mut i18n = i18n();
    i18n.set_language(language.into());

    return rsx! {
        select {
            class: "appearance-none rounded-none bg-white border-gray-300 p-1 border-solid border dark:bg-gray-700 dark:text-gray-300",
            onchange: move |evt| {
                let selected_value = evt.value().clone();
                let selected_lang: Language = selected_value.as_str().into();
                let mut new_state = STATE.read().clone();
                new_state.language = selected_lang;
                *STATE.write() = new_state;
                i18n.set_language(selected_lang.into());
                debug!("Language switched to {:?}", selected_lang);
            },
            {
                get_lang_list(STATE.read().language)
                    .iter()
                    .map(|(indicator, short, is_selected)| rsx! {
                        option { value: "{short.0}", selected: "{is_selected}", "{indicator.0} {short.0}" }
                    })
            }
        }
    };
}
