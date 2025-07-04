//! Miscellaneous utility functions for web front-end.
use crate::Gravity;
use crate::Theme;

pub fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

pub fn document() -> web_sys::Document {
    window().document().unwrap()
}

pub fn set_page_title(title: &str) {
    document().set_title(title);
}

pub fn set_body_classes(classes: &str) {
    let body = document().body().unwrap();
    body.set_class_name(classes);
}

pub fn get_base_href() -> String {
    let base = document().get_elements_by_tag_name("base").item(0);
    match base {
        Some(base) => base.get_attribute("href").unwrap(),
        None => "/".to_string(),
    }
}

fn set_classes(class: &str) {
    document()
        .document_element()
        .unwrap()
        .class_list()
        .add_1(class)
        .unwrap();
}

fn remove_classes(class: &str) {
    document()
        .document_element()
        .unwrap()
        .class_list()
        .remove_1(class)
        .unwrap();
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

pub fn set_theme_classes(theme: &Theme) {
    match theme {
        Theme::Light => {
            remove_classes("dark");
        }
        Theme::Dark => {
            set_classes("dark");
        }
    }
}

pub fn format_gravity(g: &Gravity) -> String {
    match g {
        Gravity::Plato(v) => format!("{v:.1}"),
        Gravity::Brix(v) => format!("{v:.1}"),
        Gravity::Oechsle(v) => format!("{v:.1}"),
        Gravity::SG(v) => format!("{v:.3}"),
    }
}
