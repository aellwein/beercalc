import React from "react";
import { useTranslation } from "react-i18next";
import { connect } from "react-redux";
import { convertUnits } from "../units/calculations";
import HopsAdder from "./HopsAdder";
import IbuPreset from "./IbuPreset";

const IbuCalc = (_) => {
    const { t } = useTranslation();
    return (
        <div className="flex flex-col gap-4">
            <div className="text-2xl my-3">{t('ibu calculator')}</div>
            <div><IbuPreset /></div>
            <div><HopsAdder /></div>
        </div>
    );
}

export const recalcIbu = (state) => {
    const og = convertUnits(state.gravity.original, state.unit);
    // isomerization speed factor + time
    const isoSpeedFactor = 0.046 * Math.exp(0.031 * state.ibu.flameoutTemp);
    const addIsoTime = isoSpeedFactor * state.ibu.flameout;
    for (let h of state.ibu.hops) {
        // better alpha acid isomerization through pellets
        const hopsFactor = (h.form === 'whole') ? 1.1 : 1.0;
        const ibu = (hopsFactor * ((h.amount * h.alpha * 10) / state.ibu.volume) * (1.65 * Math.pow(0.000125, (0.004 * og.plato))) * ((1 - Math.exp(-0.04 * (h.boil + addIsoTime))) / 4.15)).toFixed(1);
        h.ibu = ibu;
    }
}

const mapStateToProps = (state, _) => {
    return state.beerCalc;
}

export default connect(mapStateToProps, {})(IbuCalc);