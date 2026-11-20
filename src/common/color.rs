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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_unit_compares_only_the_variant() {
        assert!(ColorUnit::Ebc(1.0).match_unit(&ColorUnit::Ebc(9.0)));
        assert!(!ColorUnit::Ebc(1.0).match_unit(&ColorUnit::Srm(1.0)));
    }

    #[test]
    fn update_value_keeps_the_unit() {
        assert_eq!(ColorUnit::Srm(2.0).update_value(5.0).value(), 5.0);
        assert!(matches!(
            ColorUnit::Srm(2.0).update_value(5.0),
            ColorUnit::Srm(_)
        ));
    }

    #[test]
    fn same_unit_conversion_is_identity() {
        assert_eq!(ColorUnit::Ebc(12.0).to_ebc().value(), 12.0);
        assert_eq!(ColorUnit::Srm(6.0).to_srm().value(), 6.0);
        assert_eq!(ColorUnit::Lovibond(3.0).to_lovibond().value(), 3.0);
    }

    #[test]
    fn ebc_and_srm_convert_both_ways() {
        assert!((ColorUnit::Ebc(19.7).to_srm().value() - 10.0).abs() < 0.01);
        assert!((ColorUnit::Srm(10.0).to_ebc().value() - 19.7).abs() < 1e-9);
    }

    #[test]
    fn translator_keys_are_stable() {
        assert_eq!(ColorUnit::Ebc(0.0).translator_key(), "ebc");
        assert_eq!(ColorUnit::Srm(0.0).translator_key(), "srm");
        assert_eq!(ColorUnit::Lovibond(0.0).translator_key(), "lovibond");
    }
}
