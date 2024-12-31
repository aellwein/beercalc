use crate::common::prelude::*;
use anyhow::{anyhow, Result};

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
    oechsle / 4.25
}

pub fn brix_to_oechsle(brix: f64) -> f64 {
    brix * 4.25
}

pub fn oechsle_to_sg(oechsle: f64) -> f64 {
    brix_to_sg(oechsle_to_brix(oechsle))
}

pub fn oechsle_to_plato(oechsle: f64) -> f64 {
    brix_to_plato(oechsle_to_brix(oechsle))
}

pub fn plato_to_oechsle(plato: f64) -> f64 {
    brix_to_oechsle(plato_to_brix(plato))
}

pub fn sg_to_oechsle(sg: f64) -> f64 {
    brix_to_oechsle(sg_to_brix(sg))
}

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
    Ok(1.0000 - 0.00085683 * og_plato + 0.0034941 * fg_plato)
}

pub fn calc_real_extract(original_gravity: &Gravity, apparent_extract_plato: f64) -> f64 {
    0.1808 * original_gravity.to_plato().value() + 0.8192 * apparent_extract_plato
}

pub fn calc_apparent_attenuation(
    original_gravity: &Gravity,
    apparent_extract_plato: f64,
) -> Result<f64> {
    let og_plato = original_gravity.to_plato().value();
    if og_plato == 0.0 {
        return Err(anyhow!("original gravity is zero"));
    }
    Ok((1.0 - apparent_extract_plato / og_plato) * 100.0)
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
    let apparent_extract_plato;
    if matches!(original_gravity, Gravity::Brix(_)) {
        // refractometer method is used here with SG conversion from Brix units
        let apparent_extract_sg = calc_apparent_extract_sg(original_gravity, final_gravity)?;
        apparent_extract_plato = sg_to_plato(apparent_extract_sg);
    } else if matches!(final_gravity, Gravity::Plato(_)) {
        // standard method for calculation with Plato units
        apparent_extract_plato = final_gravity.to_plato().value();
    } else {
        // calculation not possible for this combination of units
        return Err(anyhow!("calculation not possible"));
    }
    let real_extract_plato = calc_real_extract(original_gravity, apparent_extract_plato);
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
    let real_extract_plato = calc_real_extract(original_gravity, apparent_extract_plato);
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
