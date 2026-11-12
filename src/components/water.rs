use crate::common::prelude::*;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

fn update_water<F>(mutate: F)
where
    F: FnOnce(&mut WaterInput),
{
    let mut new_state = STATE.read().clone();
    mutate(&mut new_state.water);
    *STATE.write() = new_state;
}

fn result_row(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 items-baseline",
            div { class: "grow", {label} }
            div { class: "text-right", {value} }
        }
    }
}

#[component]
pub fn WaterCalculator() -> Element {
    let w = STATE.read().water;
    let result = mash_water(&w);

    let result_rows = match result {
        Ok(m) => rsx! {
            {result_row(t!("ra_source").to_string(), format!("{:.2} mmol/l = {:.1} °dH", m.ra_source_mmol, mmol_to_dh(m.ra_source_mmol)))}
            {result_row(t!("ra_mash").to_string(), format!("{:.2} mmol/l = {:.1} °dH", m.ra_mash_mmol, mmol_to_dh(m.ra_mash_mmol)))}
            {result_row("Ca".to_string(), format!("{:.1} mg/l", m.ca_mg))}
            {result_row("Mg".to_string(), format!("{:.1} mg/l", m.mg_mg))}
            {result_row("SO₄".to_string(), format!("{:.1} mg/l", m.so4_mg))}
            {result_row("Cl".to_string(), format!("{:.1} mg/l", m.cl_mg))}
            {result_row("Na".to_string(), format!("{:.1} mg/l", m.na_mg))}
            {result_row("HCO₃".to_string(), format!("{:.1} mg/l", m.hco3_mg))}
        },
        Err(_) => rsx! {
            div { class: "text-red-400", {t!("water_volume_required")} }
        },
    };

    rsx! {
        Fragment {
            Header { active: Route::WaterCalculator }
            div { class: "flex flex-col gap-4 dark:text-gray-400",
                div { class: "text-2xl my-3", {t!("water_calculator")} }
                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
                    div { class: "text-xl", {t!("source_water")} }
                    {number_field(t!("hco3_label").to_string(), w.hco3_mmol, (0.0, 50.0, 0.1), "mmol/l", |v| update_water(|x| x.hco3_mmol = v))}
                    {number_field(t!("calcium").to_string(), w.ca_mg, (0.0, 1000.0, 1.0), "mg/l", |v| update_water(|x| x.ca_mg = v))}
                    {number_field(t!("magnesium").to_string(), w.mg_mg, (0.0, 1000.0, 1.0), "mg/l", |v| update_water(|x| x.mg_mg = v))}
                    {number_field(t!("sulfate").to_string(), w.so4_mg, (0.0, 1000.0, 1.0), "mg/l", |v| update_water(|x| x.so4_mg = v))}
                    {number_field(t!("chloride").to_string(), w.cl_mg, (0.0, 1000.0, 1.0), "mg/l", |v| update_water(|x| x.cl_mg = v))}
                    {number_field(t!("sodium").to_string(), w.na_mg, (0.0, 1000.0, 1.0), "mg/l", |v| update_water(|x| x.na_mg = v))}
                }
                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
                    div { class: "text-xl", {t!("mash_volume")} }
                    {number_field(t!("mash_volume").to_string(), w.volume_l, (1.0, 1000.0, 0.5), "L", |v| update_water(|x| x.volume_l = v))}
                }
                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
                    div { class: "text-xl", {t!("salts")} }
                    {number_field("CaSO₄ · 2H₂O".to_string(), w.caso4_g, (0.0, 1000.0, 0.1), "g", |v| update_water(|x| x.caso4_g = v))}
                    {number_field("CaCl₂ · 2H₂O".to_string(), w.cacl2_g, (0.0, 1000.0, 0.1), "g", |v| update_water(|x| x.cacl2_g = v))}
                    {number_field("MgSO₄ · 7H₂O".to_string(), w.mgso4_g, (0.0, 1000.0, 0.1), "g", |v| update_water(|x| x.mgso4_g = v))}
                    {number_field("NaCl".to_string(), w.nacl_g, (0.0, 1000.0, 0.1), "g", |v| update_water(|x| x.nacl_g = v))}
                    {number_field("NaHCO₃".to_string(), w.nahco3_g, (0.0, 1000.0, 0.1), "g", |v| update_water(|x| x.nahco3_g = v))}
                }
                div { class: "flex flex-col gap-4 shadow-md dark:shadow-slate-600 p-4",
                    div { class: "text-xl", {t!("lactic_acid")} }
                    {number_field(t!("lactic_acid_ml").to_string(), w.lactic_ml, (0.0, 100.0, 0.1), "ml", |v| update_water(|x| x.lactic_ml = v))}
                    {number_field(t!("lactic_concentration").to_string(), w.lactic_percent, (10.0, 90.0, 1.0), "%", |v| update_water(|x| x.lactic_percent = v))}
                }
                div { class: "flex flex-col gap-2 shadow-md dark:shadow-slate-600 p-4",
                    div { class: "text-xl", {t!("mash_water_profile")} }
                    {result_rows}
                }
            }
        }
    }
}
