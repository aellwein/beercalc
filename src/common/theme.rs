use crate::common::prelude::*;
use serde::{Deserialize, Serialize};

pub const LT_TEXT_CLASSES: &str = "bg-white text-black";
pub const DT_TEXT_CLASSES: &str = "dark:bg-neutral-700 dark:text-neutral-500";
pub const LT_ANCHOR_CLASSES: &str = "text-blue-600 hover:underline";
pub const DT_ANCHOR_CLASSES: &str = "dark:text-indigo-400 dark:hover:underline";
pub const LT_ANCHOR_CLASSES_ACTIVE: &str = "text-neutral-600 cursor-default";
pub const DT_ANCHOR_CLASSES_ACTIVE: &str = "dark:text-neutral-400 dark:cursor-default";

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
