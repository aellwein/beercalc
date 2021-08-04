import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { alcoholTerrillFormula } from '../units/calculations';

const calcNotPossible = (t) => {
    return (
        <div className="flex flex-wrap flex-col shadow-md p-4 gap-2 flex-grow items-center">
            <div className="text-center flex-grow text-red-400 table"><span className="table-cell align-middle">{t('calculation not possible')}</span></div>
        </div>
    )
}

const calcTerrill = (props, t) => {

    if (props.gravity.original <= props.gravity.final) {
        return calcNotPossible(t);
    }

    const calc = alcoholTerrillFormula(props.gravity.original, props.gravity.final, props.unit);
    if (calc.apparentExtract.toFixed(1) <= 0.0 || calc.apparentAttenuation.toFixed(1) >= 100.0) {
        return calcNotPossible(t);
    }

    return (
        <div className="flex flex-wrap flex-col shadow-md p-4 gap-2 flex-grow">
            <div className="text-2xl text-center my-3">{t('terrill equation')}</div>
            <div className="flex flex-row flex-wrap gap-2">
                <div className="text-left">{t('apparent extract')}:</div>
                <div className="text-right flex-grow flex-shrink">{calc.apparentExtract.toFixed(1)} °P</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2">
                <div className="text-left">{t('real extract')}:</div>
                <div className="text-right flex-grow flex-shrink">{calc.realExtract.toFixed(1)} %</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2">
                <div className="text-left">{t('apparent attenuation')}:</div>
                <div className="text-right flex-grow flex-shrink">{calc.apparentAttenuation.toFixed(1)} %</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2">
                <div className="text-left">{t('real attenuation')}:</div>
                <div className="text-right flex-grow flex-shrink">{calc.realAttenuation.toFixed(1)} %</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2">
                <div className="text-left">{t('alcohol by weight')}:</div>
                <div className="text-right flex-grow flex-shrink">{calc.alcByWeight.toFixed(1)} %</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2">
                <div className="text-left">{t('alcohol by volume')}:</div>
                <div className="text-right flex-grow flex-shrink">{calc.alcByVolume.toFixed(1)} %</div>
            </div>
        </div>
    );
}

const CalcTerrill = (props) => {
    const { t } = useTranslation();

    if (!props.unit || !props.gravity) {
        return <div></div>;
    }
    if (props.unit !== 'brix') {
        // the Terrill formula doesn't work for extracts taken in Plato
        // (refractometer only)
        return <div></div>;
    }
    return calcTerrill(props, t);
}


const mapStateToProps = (state) => {
    return state.beerCalc;
}

export default connect(mapStateToProps, {})(CalcTerrill);