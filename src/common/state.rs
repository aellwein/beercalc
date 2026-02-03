use crate::common::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AppState {
    pub theme: Theme,
    pub language: Language,
    pub chosen_unit: Gravity,
    pub original_gravity: Gravity,
    pub final_gravity: Gravity,
    pub ibu: Ibu,
    pub grain: Grain,
    pub brewhouse: Brewhouse,
    pub color_conversion: ColorConversion,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Ibu {
    pub boiling: f64,
    pub volume: f64,
    pub flameout: f64,
    pub flameout_temp: f64,
    pub hops: Vec<Hops>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Grain {
    pub malt: Vec<Malt>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]

pub enum HopsForm {
    Whole,
    Pellets,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Hops {
    pub form: HopsForm,
    pub amount: f64,
    pub alpha: f64,
    pub boil_time: f64,
    pub ibu: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Malt {
    pub amount: f64,
    pub mass_unit: MassUnit,
    pub color: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum MassUnit {
    Kilogram,
    Gram,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum VolumeMeasuredAt {
    HundredDegreesCelsius,
    TwentyDegreesCelsius,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Brewhouse {
    pub grain_mass: f64,
    pub grain_mass_unit: MassUnit,
    pub volume_measured_at: VolumeMeasuredAt,
    pub wort_volume: f64,
    pub grain_mass_from_brewhouse: GrainMassFromBrewhouse,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GrainMassFromBrewhouse {
    pub original_gravity: Gravity,
    pub wort_volume: f64,
    pub volume_measured_at: VolumeMeasuredAt,
    pub brewhouse_efficiency: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ColorConversion {
    pub color: ColorUnit,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            theme: get_preferred_theme(),
            language: get_preferred_language(),
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
            color_conversion: ColorConversion {
                color: ColorUnit::Ebc(12.0),
            },
        }
    }
}
