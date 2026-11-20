use crate::common::prelude::*;
use serde::{Deserialize, Serialize};

pub const LT_TEXT_CLASSES: &str = "bg-page text-ink";
pub const DT_TEXT_CLASSES: &str = "";
pub const LT_ANCHOR_CLASSES: &str = "text-accent hover:underline";
pub const DT_ANCHOR_CLASSES: &str = "";
pub const LT_ANCHOR_CLASSES_ACTIVE: &str = "bg-accent text-accent-ink";

/// Surface of a calculator section: rounded card with a border and a soft shadow.
pub const CARD_CLASS: &str = "rounded-2xl border border-line bg-card p-5 shadow-sm";
/// Number and text inputs.
pub const INPUT_CLASS: &str = "rounded-md border border-line bg-field px-2 py-1 text-ink outline-none focus:border-accent focus:ring-2 focus:ring-accent/30";
/// Dropdowns.
pub const SELECT_CLASS: &str = "rounded-md border border-line bg-field px-2 py-1 text-ink outline-none appearance-none focus:border-accent focus:ring-2 focus:ring-accent/30";
/// Main action, e.g. "add hops".
pub const BUTTON_PRIMARY_CLASS: &str = "rounded-full bg-accent px-4 py-2 text-sm font-medium text-accent-ink shadow-sm hover:opacity-90";
/// Destructive small action, e.g. removing a hop addition.
pub const BUTTON_DANGER_CLASS: &str =
    "rounded-full bg-danger px-3 py-1 text-sm font-medium text-white shadow-sm hover:opacity-90";
/// Page heading.
pub const TITLE_CLASS: &str = "text-3xl font-semibold tracking-tight text-ink";
/// Section heading inside a card.
pub const SECTION_TITLE_CLASS: &str = "text-lg font-semibold text-ink";

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Theme {
    Light,
    Dark,
}

pub fn set_theme(theme: &Theme) {
    match *theme {
        Theme::Dark => set_classes("dark"),
        Theme::Light => remove_classes("dark"),
    };
}
