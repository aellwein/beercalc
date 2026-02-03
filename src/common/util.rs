use crate::common::prelude::*;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("unable to get window element")
}

pub fn document() -> web_sys::Document {
    window().document().expect("unable to get document element")
}

pub fn get_preferred_theme() -> Theme {
    match window()
        .match_media("(prefers-color-scheme: dark)")
        .expect("unable to query media list")
    {
        Some(x) => {
            if x.matches() {
                Theme::Dark
            } else {
                Theme::Light
            }
        }
        None => Theme::Light,
    }
}

pub fn set_classes(class: &str) {
    document()
        .document_element()
        .expect("unable to get body element")
        .class_list()
        .add_1(class)
        .expect("unable to add class");
}

pub fn remove_classes(class: &str) {
    document()
        .document_element()
        .expect("unable to get body element")
        .class_list()
        .remove_1(class)
        .expect("unable to remove class");
}

pub fn get_preferred_language() -> Language {
    let navigator = window().navigator();
    let languages: Vec<Language> = navigator
        .languages()
        .iter()
        .map(|j| {
            let s_lang = j.as_string().unwrap().split("-").collect::<Vec<&str>>()[0].to_string();
            let r = s_lang.try_into();
            match r {
                Ok(lang) => lang,
                Err(_) => Language::English,
            }
        })
        .collect();
    //console::log_1(&format!("Languages: {:?}", languages).into());
    if !languages.is_empty() {
        languages[0]
    } else {
        Language::English
    }
}
