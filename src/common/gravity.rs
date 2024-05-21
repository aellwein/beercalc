use crate::common::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Gravity {
    Plato(f64),
    Brix(f64),
    SG(f64),
    Oechsle(f64),
}

impl Gravity {
    pub fn value(&self) -> f64 {
        match self {
            Gravity::Plato(v) => v.clone(),
            Gravity::Brix(v) => v.clone(),
            Gravity::SG(v) => v.clone(),
            Gravity::Oechsle(v) => v.clone(),
        }
    }

    pub fn update_value(&self, newval: f64) -> Self {
        match self {
            Gravity::Plato(_) => Gravity::Plato(newval),
            Gravity::Brix(_) => Gravity::Brix(newval),
            Gravity::SG(_) => Gravity::SG(newval),
            Gravity::Oechsle(_) => Gravity::Oechsle(newval),
        }
    }

    pub fn to_brix(&self) -> Self {
        match self {
            Gravity::Brix(v) => Gravity::Brix((*v).clone()),
            Gravity::Plato(v) => Gravity::Brix(plato_to_brix((*v).clone())),
            Gravity::SG(v) => Gravity::Brix(sg_to_brix((*v).clone())),
            Gravity::Oechsle(v) => Gravity::Brix(oechsle_to_brix((*v).clone())),
        }
    }

    pub fn to_plato(&self) -> Self {
        match self {
            Gravity::Plato(v) => Gravity::Plato((*v).clone()),
            Gravity::Brix(v) => Gravity::Plato(brix_to_plato((*v).clone())),
            Gravity::SG(v) => Gravity::Plato(sg_to_plato((*v).clone())),
            Gravity::Oechsle(v) => Gravity::Plato(oechsle_to_plato((*v).clone())),
        }
    }
    pub fn to_sg(&self) -> Self {
        match self {
            Gravity::SG(v) => Gravity::SG((*v).clone()),
            Gravity::Brix(v) => Gravity::SG(brix_to_sg((*v).clone())),
            Gravity::Plato(v) => Gravity::SG(plato_to_sg((*v).clone())),
            Gravity::Oechsle(v) => Gravity::SG(oechsle_to_sg((*v).clone())),
        }
    }
    pub fn to_oechsle(&self) -> Self {
        match self {
            Gravity::Oechsle(v) => Gravity::Oechsle((*v).clone()),
            Gravity::Brix(v) => Gravity::Oechsle(brix_to_oechsle((*v).clone())),
            Gravity::Plato(v) => Gravity::Oechsle(plato_to_oechsle((*v).clone())),
            Gravity::SG(v) => Gravity::Oechsle(sg_to_oechsle((*v).clone())),
        }
    }
}
