//! Miscellaneous utility functions for web front-end.
use web_sys::wasm_bindgen::JsValue;

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

pub fn set_body_classes(class: &str) {
    let classes = class
        .split_whitespace()
        .map(JsValue::from_str)
        .collect::<web_sys::js_sys::Array>();
    document()
        .body()
        .expect("unable to get body element")
        .class_list()
        .add(&classes)
        .expect("unable to add class");
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
    window()
        .navigator()
        .languages()
        .iter()
        .filter_map(|j| j.as_string())
        .find_map(|tag| {
            let short = tag.split('-').next()?.to_string();
            short.try_into().ok()
        })
        .unwrap_or(Language::English)
}

pub fn get_commit_hash() -> String {
    include_str!("../../assets/commit_sha.txt")
        .trim()
        .to_string()
}
