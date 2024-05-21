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
