use crate::common::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorUnit {
    Ebc(f64),
    Srm(f64),
    Lovibond(f64),
}

impl ColorUnit {
    pub fn match_unit(&self, other: &ColorUnit) -> bool {
        match self {
            ColorUnit::Ebc(_) => matches!(other, ColorUnit::Ebc(_)),
            ColorUnit::Srm(_) => matches!(other, ColorUnit::Srm(_)),
            ColorUnit::Lovibond(_) => matches!(other, ColorUnit::Lovibond(_)),
        }
    }
    pub fn value(&self) -> f64 {
        match self {
            ColorUnit::Ebc(v) => *v,
            ColorUnit::Srm(v) => *v,
            ColorUnit::Lovibond(v) => *v,
        }
    }
    pub fn update_value(&self, newval: f64) -> Self {
        match self {
            ColorUnit::Ebc(_) => ColorUnit::Ebc(newval),
            ColorUnit::Srm(_) => ColorUnit::Srm(newval),
            ColorUnit::Lovibond(_) => ColorUnit::Lovibond(newval),
        }
    }

    pub fn to_ebc(&self) -> Self {
        match self {
            ColorUnit::Ebc(v) => ColorUnit::Ebc(*v),
            ColorUnit::Srm(v) => ColorUnit::Ebc(srm_to_ebc(*v)),
            ColorUnit::Lovibond(v) => ColorUnit::Ebc(lovibond_to_ebc(*v)),
        }
    }
    pub fn to_srm(&self) -> Self {
        match self {
            ColorUnit::Srm(v) => ColorUnit::Srm(*v),
            ColorUnit::Ebc(v) => ColorUnit::Srm(ebc_to_srm(*v)),
            ColorUnit::Lovibond(v) => ColorUnit::Srm(lovibond_to_srm(*v)),
        }
    }
    pub fn to_lovibond(&self) -> Self {
        match self {
            ColorUnit::Lovibond(v) => ColorUnit::Lovibond(*v),
            ColorUnit::Ebc(v) => ColorUnit::Lovibond(ebc_to_lovibond(*v)),
            ColorUnit::Srm(v) => ColorUnit::Lovibond(srm_to_lovibond(*v)),
        }
    }
    pub fn translator_key(&self) -> String {
        match self {
            ColorUnit::Ebc(_) => "ebc",
            ColorUnit::Srm(_) => "srm",
            ColorUnit::Lovibond(_) => "lovibond",
        }
        .to_string()
    }
}
