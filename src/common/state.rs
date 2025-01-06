use crate::common::prelude::*;
use enum_iterator::Sequence;
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
    pub ibu: Ibu,
    pub grain: Grain,
    pub brewhouse: Brewhouse,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Ibu {
    pub boiling: f64,
    pub volume: f64,
    pub flameout: f64,
    pub flameout_temp: f64,
    pub hops: Vec<Hops>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Grain {
    pub malt: Vec<Malt>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Sequence)]
pub enum HopsForm {
    Whole,
    Pellets,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Hops {
    pub form: HopsForm,
    pub amount: f64,
    pub alpha: f64,
    pub boil_time: f64,
    pub ibu: f64,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Malt {
    pub amount: f64,
    pub mass_unit: MassUnit,
    pub color: f64,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Sequence)]
pub enum MassUnit {
    Kilogram,
    Gram,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum VolumeMeasuredAt {
    HundredDegreesCelsius,
    TwentyDegreesCelsius,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Brewhouse {
    pub grain_mass: f64,
    pub grain_mass_unit: MassUnit,
    pub volume_measured_at: VolumeMeasuredAt,
    pub wort_volume: f64,
    pub grain_mass_from_brewhouse: GrainMassFromBrewhouse,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct GrainMassFromBrewhouse {
    pub original_gravity: Gravity,
    pub wort_volume: f64,
    pub volume_measured_at: VolumeMeasuredAt,
    pub brewhouse_efficiency: f64,
}

impl Default for CalcState {
    fn default() -> Self {
        CalcState {
            theme: get_preferred_theme(),
            language: Translator::get_preferred_language(),
            chosen_unit: Gravity::Brix(0.0),
            original_gravity: Gravity::Brix(12.0),
            final_gravity: Gravity::Brix(6.0),
            ibu: Ibu {
                boiling: 90.0,
                volume: 50.0,
                flameout: 5.0,
                flameout_temp: 85.0,
                hops: vec![],
            },
            grain: Grain { malt: vec![] },
            brewhouse: Brewhouse {
                volume_measured_at: VolumeMeasuredAt::HundredDegreesCelsius,
                grain_mass: 5.0,
                grain_mass_unit: MassUnit::Kilogram,
                wort_volume: 28.0,
                grain_mass_from_brewhouse: GrainMassFromBrewhouse {
                    original_gravity: Gravity::Brix(12.0),
                    wort_volume: 28.0,
                    volume_measured_at: VolumeMeasuredAt::HundredDegreesCelsius,
                    brewhouse_efficiency: 65.6,
                },
            },
        }
    }
}

impl HopsForm {
    pub fn translator_key(&self) -> String {
        match self {
            HopsForm::Whole => "whole".to_string(),
            HopsForm::Pellets => "pellets".to_string(),
        }
    }
}

impl From<String> for HopsForm {
    fn from(value: String) -> Self {
        match value.as_str() {
            "whole" => HopsForm::Whole,
            "pellets" => HopsForm::Pellets,
            _ => HopsForm::Whole,
        }
    }
}

impl MassUnit {
    pub fn translator_key(&self) -> String {
        match self {
            MassUnit::Kilogram => "kg".to_string(),
            MassUnit::Gram => "g".to_string(),
        }
    }
}

impl From<String> for MassUnit {
    fn from(value: String) -> Self {
        match value.as_str() {
            "kg" => MassUnit::Kilogram,
            "g" => MassUnit::Gram,
            _ => MassUnit::Kilogram,
        }
    }
}
