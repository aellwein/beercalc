#![allow(dead_code)]
//! This module contains functions for various calculations related to brewing.
use crate::common::prelude::*;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

pub fn brix_to_plato(brix: f64) -> f64 {
    brix / 1.03
}

pub fn plato_to_brix(plato: f64) -> f64 {
    plato * 1.03
}

pub fn plato_to_sg(plato: f64) -> f64 {
    1.0 + (plato / (258.6 - ((plato / 258.2) * 227.1)))
}

pub fn sg_to_plato(sg: f64) -> f64 {
    668.72 * sg - 463.37 - 205.347 * sg.powi(2)
}

pub fn sg_to_brix(sg: f64) -> f64 {
    plato_to_brix(sg_to_plato(sg))
}

pub fn brix_to_sg(brix: f64) -> f64 {
    plato_to_sg(brix_to_plato(brix))
}

pub fn oechsle_to_brix(oechsle: f64) -> f64 {
    sg_to_brix(oechsle_to_sg(oechsle))
}

pub fn brix_to_oechsle(brix: f64) -> f64 {
    sg_to_oechsle(brix_to_sg(brix))
}

pub fn oechsle_to_sg(oechsle: f64) -> f64 {
    1.0 + oechsle / 1000.0
}

pub fn oechsle_to_plato(oechsle: f64) -> f64 {
    brix_to_plato(oechsle_to_brix(oechsle))
}

pub fn plato_to_oechsle(plato: f64) -> f64 {
    brix_to_oechsle(plato_to_brix(plato))
}

pub fn sg_to_oechsle(sg: f64) -> f64 {
    (sg - 1.0) * 1000.0
}

#[derive(Clone, PartialEq)]
pub struct AlcCalcResult {
    pub apparent_extract: f64,
    pub real_extract: f64,
    pub apparent_attenuation: f64,
    pub real_attenuation: f64,
    pub alcohol_by_weight: f64,
    pub alcohol_by_volume: f64,
}

pub fn calc_apparent_extract_sg(
    original_gravity: &Gravity,
    final_gravity: &Gravity,
) -> Result<f64> {
    let og_plato = original_gravity.to_plato().value();
    let fg_brix = final_gravity.to_brix().value();

    if og_plato == 0.0 {
        return Err(anyhow!("original gravity is zero"));
    }

    let apparent_extract_sg = 1.001843
        - 0.002318474 * og_plato
        - 0.000007775 * og_plato.powi(2)
        - 0.000000034 * og_plato.powi(3)
        + 0.00574 * fg_brix
        + 0.00003344 * fg_brix.powi(2)
        + 0.000000086 * fg_brix.powi(3);

    if apparent_extract_sg <= 0.0 {
        return Err(anyhow!("apparent extract is less or equal zero"));
    }
    Ok(apparent_extract_sg)
}

pub fn calc_apparent_extract_sg_terrill(
    original_gravity: &Gravity,
    final_gravity: &Gravity,
) -> Result<f64> {
    let og_plato = original_gravity.to_plato().value();
    let fg_plato = final_gravity.to_plato().value();
    if og_plato == 0.0 {
        return Err(anyhow!("original gravity is zero"));
    }
    if og_plato <= fg_plato {
        return Err(anyhow!(
            "original gravity is less or equal than final gravity"
        ));
    }
    let result = 1.0000 - 0.00085683 * og_plato + 0.0034941 * fg_plato;
    if result <= 0.0 {
        return Err(anyhow!("apparent extract is less or equal zero"));
    }
    Ok(result)
}

pub fn calc_real_extract(original_gravity: &Gravity, apparent_extract_plato: f64) -> Result<f64> {
    let result = 0.1808 * original_gravity.to_plato().value() + 0.8192 * apparent_extract_plato;
    match result {
        r if r <= 0.0 => Err(anyhow!("real extract is less or equal zero")),
        r if r >= 100.0 => Err(anyhow!("real extract is greater or equal 100")),
        _ => Ok(result),
    }
}

pub fn calc_apparent_attenuation(
    original_gravity: &Gravity,
    apparent_extract_plato: f64,
) -> Result<f64> {
    let og_plato = original_gravity.to_plato().value();
    if og_plato == 0.0 {
        return Err(anyhow!("original gravity is zero"));
    }
    let result = (1.0 - apparent_extract_plato / og_plato) * 100.0;
    match result {
        r if r <= 0.0 => Err(anyhow!("apparent attenuation is less or equal zero")),
        r if r >= 100.0 => Err(anyhow!("apparent attenuation is greater or equal 100")),
        _ => Ok(result),
    }
}

pub fn calc_real_attenuation(original_gravity: &Gravity, real_extract_plato: f64) -> Result<f64> {
    let og_plato = original_gravity.to_plato().value();
    if og_plato == 0.0 {
        return Err(anyhow!("original gravity is zero"));
    }
    Ok((1.0 - real_extract_plato / og_plato) * 100.0)
}

pub fn calc_alcohol_by_weight(original_gravity: &Gravity, real_extract_plato: f64) -> Result<f64> {
    let og_plato = original_gravity.to_plato().value();
    if og_plato == 0.0 {
        return Err(anyhow!("original gravity is zero"));
    }
    if (og_plato - real_extract_plato) <= 0.0 {
        return Err(anyhow!("gravity difference is less or equal zero"));
    }
    let div = 2.0665 - (0.010665 * og_plato);
    if div <= 0.0 {
        return Err(anyhow!("division by zero"));
    }
    Ok((og_plato - real_extract_plato) / div)
}

pub fn calc_alcohol_by_volume(original_gravity: &Gravity, real_extract_plato: f64) -> Result<f64> {
    let og_plato = original_gravity.to_plato().value();
    if og_plato == 0.0 {
        return Err(anyhow!("original gravity is zero"));
    }
    if (og_plato - real_extract_plato) <= 0.0 {
        return Err(anyhow!("gravity difference is less or equal zero"));
    }
    let div = 2.0665 - (0.010665 * og_plato);
    if div <= 0.0 {
        return Err(anyhow!("division by zero"));
    }
    Ok(1.0 / 0.79 * (og_plato - real_extract_plato) / div)
}

pub fn calc_alcohol_standard_equation(
    original_gravity: &Gravity,
    final_gravity: &Gravity,
) -> Result<AlcCalcResult> {
    let apparent_extract_plato = if matches!(original_gravity, Gravity::Brix(_)) {
        // refractometer method is used here with SG conversion from Brix units
        let apparent_extract_sg = calc_apparent_extract_sg(original_gravity, final_gravity)?;
        sg_to_plato(apparent_extract_sg)
    } else if matches!(final_gravity, Gravity::Plato(_)) {
        // standard method for calculation with Plato units
        final_gravity.to_plato().value()
    } else {
        // calculation not possible for this combination of units
        return Err(anyhow!("calculation not possible"));
    };
    let real_extract_plato = calc_real_extract(original_gravity, apparent_extract_plato)?;
    let apparent_attenuation_plato =
        calc_apparent_attenuation(original_gravity, apparent_extract_plato)?;
    let real_attenuation_plato = calc_real_attenuation(original_gravity, real_extract_plato)?;
    let alcohol_by_weight = calc_alcohol_by_weight(original_gravity, real_extract_plato)?;
    let alcohol_by_volume = calc_alcohol_by_volume(original_gravity, real_extract_plato)?;

    Ok(AlcCalcResult {
        apparent_extract: apparent_extract_plato,
        real_extract: real_extract_plato,
        apparent_attenuation: apparent_attenuation_plato,
        real_attenuation: real_attenuation_plato,
        alcohol_by_weight,
        alcohol_by_volume,
    })
}

pub fn calc_alcohol_terrill_equation(
    original_gravity: &Gravity,
    final_gravity: &Gravity,
) -> Result<AlcCalcResult> {
    if !matches!(original_gravity, Gravity::Brix(_)) {
        return Err(anyhow!("this equation requires Brix units"));
    }
    let apparent_extract_sg = calc_apparent_extract_sg_terrill(original_gravity, final_gravity)?;
    let apparent_extract_plato = sg_to_plato(apparent_extract_sg);
    let real_extract_plato = calc_real_extract(original_gravity, apparent_extract_plato)?;
    let apparent_attenuation_plato =
        calc_apparent_attenuation(original_gravity, apparent_extract_plato)?;
    let real_attenuation_plato = calc_real_attenuation(original_gravity, real_extract_plato)?;
    let alcohol_by_weight = calc_alcohol_by_weight(original_gravity, real_extract_plato)?;
    let alcohol_by_volume = calc_alcohol_by_volume(original_gravity, real_extract_plato)?;

    Ok(AlcCalcResult {
        apparent_extract: apparent_extract_plato,
        real_extract: real_extract_plato,
        apparent_attenuation: apparent_attenuation_plato,
        real_attenuation: real_attenuation_plato,
        alcohol_by_weight,
        alcohol_by_volume,
    })
}

pub fn calculate_ebc(original_gravity: &Gravity, grain: &Vec<Malt>, boiling_time: f64) -> f64 {
    let mut total_mass_kg = 0.0;
    let og_plato = original_gravity.to_plato().value();
    let mut prod_mass_ebc = 0.0;
    for malt in grain {
        let factor = match malt.mass_unit {
            MassUnit::Kilogram => 1.0,
            MassUnit::Gram => 0.001,
        };
        total_mass_kg += malt.amount * factor;
        prod_mass_ebc += malt.color * malt.amount * factor;
    }
    (prod_mass_ebc / total_mass_kg) * og_plato / 10.0 + 1.5 * boiling_time / 60.0
}

pub fn calculate_brewhouse_efficiency(original_gravity: &Gravity, brewhouse: &Brewhouse) -> f64 {
    let og_plato = original_gravity.to_plato().value();
    let og_sg = plato_to_sg(og_plato);
    let temp_factor = match brewhouse.volume_measured_at {
        VolumeMeasuredAt::HundredDegreesCelsius => 0.96,
        VolumeMeasuredAt::TwentyDegreesCelsius => 1.0,
    };
    let grain_mass_kg = match brewhouse.grain_mass_unit {
        MassUnit::Kilogram => brewhouse.grain_mass,
        MassUnit::Gram => brewhouse.grain_mass * 0.001,
    };
    (brewhouse.wort_volume * og_sg * (og_plato / 100.0) * temp_factor / grain_mass_kg) * 100.0
}

pub fn calculate_grain_mass_from_brewhouse(gmvb: &GrainMassFromBrewhouse) -> f64 {
    let temp_factor = match gmvb.volume_measured_at {
        VolumeMeasuredAt::HundredDegreesCelsius => 0.96,
        VolumeMeasuredAt::TwentyDegreesCelsius => 1.0,
    };
    let og_plato = gmvb.original_gravity.to_plato().value();
    let og_sg = plato_to_sg(og_plato);
    (gmvb.wort_volume * og_sg * (og_plato / 100.0) * temp_factor / gmvb.brewhouse_efficiency)
        * 100.0
}

pub fn srm_to_ebc(srm: f64) -> f64 {
    srm * 1.97
}

pub fn lovibond_to_ebc(lovibond: f64) -> f64 {
    srm_to_ebc(lovibond_to_srm(lovibond))
}

pub fn ebc_to_srm(ebc: f64) -> f64 {
    ebc * 0.508
}

pub fn lovibond_to_srm(lovibond: f64) -> f64 {
    lovibond * 1.3546 - 0.76
}

pub fn ebc_to_lovibond(ebc: f64) -> f64 {
    srm_to_lovibond(ebc_to_srm(ebc))
}

pub fn srm_to_lovibond(srm: f64) -> f64 {
    (srm + 0.76) / 1.3546
}

/// Factor `f(T)` in `CO2 [g/L] = (P [bar] + 1.013) * f(T)`, from the
/// Braukaiser carbonation table formula used by maischemalzundmehr.de.
pub fn co2_solubility_factor(temperature: f64) -> f64 {
    10.0 * (-10.73797 + 2617.25 / (temperature + 273.15)).exp()
}

/// Dissolved CO2 in beer in g/L at gauge pressure `pressure_bar` and `temperature` in °C.
pub fn dissolved_co2(pressure_bar: f64, temperature: f64) -> f64 {
    (pressure_bar + 1.013) * co2_solubility_factor(temperature)
}

/// Gauge pressure in bar needed to reach `co2_target` g/L at `temperature` in °C.
/// Fails when the target is below the CO2 dissolved at atmospheric pressure.
pub fn spunding_pressure(co2_target: f64, temperature: f64) -> Result<f64> {
    let pressure = co2_target / co2_solubility_factor(temperature) - 1.013;
    if pressure < 0.0 {
        return Err(anyhow!(
            "target CO2 is below the dissolved CO2 at atmospheric pressure"
        ));
    }
    Ok(pressure)
}

#[cfg(test)]
mod carbonation_tests {
    use super::*;

    #[test]
    fn dissolved_co2_matches_reference_values() {
        // maischemalzundmehr page / Braukaiser table: ~1.7 g/L at 20 °C, ~2.3 g/L at 10 °C, 0 bar.
        assert!((dissolved_co2(0.0, 20.0) - 1.66).abs() < 0.01);
        assert!((dissolved_co2(0.0, 10.0) - 2.27).abs() < 0.01);
    }

    #[test]
    fn spunding_pressure_round_trips_dissolved_co2() {
        for &(pressure, temperature) in &[(0.0, 20.0), (1.1, 12.0), (2.5, 8.0)] {
            let co2 = dissolved_co2(pressure, temperature);
            let back = spunding_pressure(co2, temperature).unwrap();
            assert!((back - pressure).abs() < 1e-9);
        }
    }

    #[test]
    fn spunding_pressure_rejects_target_below_atmospheric() {
        assert!(spunding_pressure(0.5, 20.0).is_err());
    }
}

pub const MW_CA: f64 = 40.08;
pub const MW_MG: f64 = 24.31;
pub const MW_NA: f64 = 22.99;
pub const MW_SO4: f64 = 96.06;
pub const MW_CL: f64 = 35.45;
pub const MW_HCO3: f64 = 61.02;

/// Residual alkalinity in mmol/l from HCO3- (mmol/l) and Ca and Mg (mg/l), using the
/// Kolbach factors 1.75 (Ca) and 3.5 (Mg) from the maischemalzundmehr water calculator.
pub fn residual_alkalinity(hco3_mmol: f64, ca_mg: f64, mg_mg: f64) -> f64 {
    hco3_mmol - (ca_mg / MW_CA) / 1.75 - (mg_mg / MW_MG) / 3.5
}

/// Converts mmol/l into German degrees of carbonate hardness (°dH), as used on the page.
pub fn mmol_to_dh(mmol: f64) -> f64 {
    mmol / 0.3566
}

/// Alkalinity reduction in mmol/l from adding `ml` of lactic acid with mass fraction
/// `concentration_percent` (%) to `volume_l` litres of water. Density 1.206 g/ml, molar mass 90.08 g/mol.
pub fn lactic_acid_reduction(ml: f64, concentration_percent: f64, volume_l: f64) -> f64 {
    ml * 1.206 * (concentration_percent / 100.0) * 1000.0 / 90.08 / volume_l
}

/// Source water and additions for a mash water volume.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WaterInput {
    pub hco3_mmol: f64,
    pub ca_mg: f64,
    pub mg_mg: f64,
    pub so4_mg: f64,
    pub cl_mg: f64,
    pub na_mg: f64,
    pub volume_l: f64,
    /// grams of CaSO4 x 2H2O, CaCl2 x 2H2O, MgSO4 x 7H2O, NaCl, NaHCO3
    pub caso4_g: f64,
    pub cacl2_g: f64,
    pub mgso4_g: f64,
    pub nacl_g: f64,
    pub nahco3_g: f64,
    pub lactic_ml: f64,
    pub lactic_percent: f64,
}

/// Resulting mash water: ion profile in mg/l and alkalinity before and after additions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MashWater {
    pub ra_source_mmol: f64,
    pub ra_mash_mmol: f64,
    pub ca_mg: f64,
    pub mg_mg: f64,
    pub so4_mg: f64,
    pub cl_mg: f64,
    pub na_mg: f64,
    pub hco3_mg: f64,
}

/// Mash water chemistry: salt additions and lactic acid applied to the source water.
pub fn mash_water(input: &WaterInput) -> Result<MashWater> {
    if input.volume_l <= 0.0 {
        return Err(anyhow!("water volume must be positive"));
    }
    let v = input.volume_l;
    // mmol/l added by each salt: grams / molar mass * 1000 / volume.
    let mmol = |grams: f64, molar_mass: f64| grams / molar_mass * 1000.0 / v;

    let caso4 = mmol(input.caso4_g, 172.17);
    let cacl2 = mmol(input.cacl2_g, 147.01);
    let mgso4 = mmol(input.mgso4_g, 246.47);
    let nacl = mmol(input.nacl_g, 58.44);
    let nahco3 = mmol(input.nahco3_g, 84.01);

    let ca_mg = input.ca_mg + (caso4 + cacl2) * MW_CA;
    let mg_mg = input.mg_mg + mgso4 * MW_MG;
    let so4_mg = input.so4_mg + (caso4 + mgso4) * MW_SO4;
    let cl_mg = input.cl_mg + (2.0 * cacl2 + nacl) * MW_CL;
    let na_mg = input.na_mg + (nacl + nahco3) * MW_NA;
    let hco3_mmol = input.hco3_mmol + nahco3;

    let ra_source_mmol = residual_alkalinity(input.hco3_mmol, input.ca_mg, input.mg_mg);
    let ra_after_salts = residual_alkalinity(hco3_mmol, ca_mg, mg_mg);
    let lactic = lactic_acid_reduction(input.lactic_ml, input.lactic_percent, v);
    let ra_mash_mmol = ra_after_salts - lactic;
    let hco3_mg = (hco3_mmol - lactic) * MW_HCO3;

    Ok(MashWater {
        ra_source_mmol,
        ra_mash_mmol,
        ca_mg,
        mg_mg,
        so4_mg,
        cl_mg,
        na_mg,
        hco3_mg,
    })
}

#[cfg(test)]
mod water_tests {
    use super::*;

    #[test]
    fn residual_alkalinity_uses_kolbach_factors() {
        // 3 mmol/l HCO3-, no hardness -> RA = 3.
        assert!((residual_alkalinity(3.0, 0.0, 0.0) - 3.0).abs() < 1e-9);
        // 45 mg Ca = 1.12 mmol, 45 mg Mg = 1.85 mmol (page examples).
        let ra = residual_alkalinity(3.0, 45.0, 45.0);
        assert!((ra - (3.0 - 1.12 / 1.75 - 1.85 / 3.5)).abs() < 0.01);
    }

    #[test]
    fn lactic_acid_matches_page_example() {
        // Page: 3.3 ml of 80% lactic acid in 100 L lowers RA by about 1 °dH.
        let dh = mmol_to_dh(lactic_acid_reduction(3.3, 80.0, 100.0));
        assert!((dh - 1.0).abs() < 0.05);
    }

    #[test]
    fn nacl_adds_sodium_and_chloride() {
        let input = WaterInput {
            volume_l: 10.0,
            nacl_g: 1.0,
            lactic_percent: 80.0,
            ..Default::default()
        };
        let mash = mash_water(&input).unwrap();
        // 1 g NaCl in 10 L = 1.711 mmol/l -> 39.3 mg/l Na and 60.7 mg/l Cl.
        assert!((mash.na_mg - 39.3).abs() < 0.1);
        assert!((mash.cl_mg - 60.7).abs() < 0.1);
    }

    #[test]
    fn zero_volume_is_rejected() {
        assert!(mash_water(&WaterInput::default()).is_err());
    }
}
