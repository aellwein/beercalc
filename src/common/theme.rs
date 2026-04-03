use crate::common::prelude::*;
use serde::{Deserialize, Serialize};

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
