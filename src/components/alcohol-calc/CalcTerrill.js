import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { alcoholTerrillFormula } from './calculations';

const calcNotPossible = (t) => {
    return (
        <div className="columns is-centered">
            <div className="column is-narrow">{t('calculation not possible')}</div>
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
        <div>
            <div className="columns">
                <div className="column no-bottom-padding">{t('apparent extract')}:</div>
                <div className="column no-bottom-padding is-narrow">{calc.apparentExtract.toFixed(1)} °P</div>
            </div>
            <div className="columns">
                <div className="column no-bottom-padding">{t('real extract')}:</div>
                <div className="column no-bottom-padding is-narrow">{calc.realExtract.toFixed(1)} %</div>
            </div>
            <div className="columns">
                <div className="column no-bottom-padding">{t('apparent attenuation')}:</div>
                <div className="column no-bottom-padding is-narrow">{calc.apparentAttenuation.toFixed(1)} %</div>
            </div>
            <div className="columns">
                <div className="column no-bottom-padding">{t('real attenuation')}:</div>
                <div className="column no-bottom-padding is-narrow">{calc.realAttenuation.toFixed(1)} %</div>
            </div>
            <div className="columns">
                <div className="column no-bottom-padding">{t('alcohol by weight')}:</div>
                <div className="column no-bottom-padding is-narrow">{calc.alcByWeight.toFixed(1)} %</div>
            </div>
            <div className="columns">
                <div className="column no-bottom-padding">{t('alcohol by volume')}:</div>
                <div className="column no-bottom-padding is-narrow">{calc.alcByVolume.toFixed(1)} %</div>
            </div>
        </div >
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
    return (
        <div className="card">
            <div className="card-content">
                <div className="columns is-centered">
                    <div className="column is-narrow">
                        <div className="title is-4">{t('terrill equation')}</div>
                    </div>
                </div>
                {calcTerrill(props, t)}
            </div>
        </div>
    );
}


const mapStateToProps = (state) => {
    return state.beerCalc;
}

export default connect(mapStateToProps, {})(CalcTerrill);