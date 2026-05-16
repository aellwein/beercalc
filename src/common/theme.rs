use crate::common::prelude::*;
use serde::{Deserialize, Serialize};

pub const LT_TEXT_CLASSES: &str = "";
pub const DT_TEXT_CLASSES: &str = "dark:bg-gray-800 dark:text-gray-400";
pub const LT_ANCHOR_CLASSES: &str = "text-indigo-600 hover:underline";
pub const DT_ANCHOR_CLASSES: &str = "dark:text-indigo-400";
pub const LT_ANCHOR_CLASSES_ACTIVE: &str = "text-indigo-600 border-b-2 border-indigo-600 pb-2";
pub const DT_ANCHOR_CLASSES_ACTIVE: &str = "dark:text-indigo-500";

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
