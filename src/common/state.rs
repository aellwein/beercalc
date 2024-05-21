use crate::common::prelude::*;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Clone, Serialize, Deserialize, PartialEq)]
#[store(storage = "local")]
pub struct CalcState {
    pub theme: Theme,
    pub language: Language,
    pub chosen_unit: Gravity,
    pub original_gravity: Gravity,
    pub final_gravity: Gravity,
}

impl Default for CalcState {
    fn default() -> Self {
        CalcState {
            theme: get_preferred_theme(),
            language: Translator::get_preferred_language(),
            chosen_unit: Gravity::Brix(0.0),
            original_gravity: Gravity::Brix(12.0),
            final_gravity: Gravity::Brix(6.0),
        }
    }
}
