import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from "react-redux";
import { boilingTime, changeUnit, originalGravity } from "../../actions";
import ColorDisplay from './ColorDisplay';
import MaltAdder from './MaltAdder';

const changeOriginalGravity = (props, newOg) => {
    let og = parseFloat(newOg);
    if (isNaN(og)) {
        return;
    }
    if (og < .1) {
        og = .1;
    }
    if (og > 40) {
        og = 40;
    }
    if (og !== null) {
        props.originalGravity(og);
    }
}

const onBoilingChange = (props, time) => {
    let boil = parseFloat(time);
    if (isNaN(boil)) {
        return;
    }
    if (boil < 1) {
        boil = 1;
    }
    if (boil > 600) {
        boil = 600;
    }
    if (boil !== null) {
        props.boilingTime(boil);
    }
}

const getOptions = function* (props, t) {
    if (props.unit === 'brix') {
        yield <option value='brix' key='brix'>{t('brix')}</option>;
        yield <option value='plato' key='plato'>{t('plato')}</option>;
    } else {
        yield <option value='plato' key='plato'>{t('plato')}</option>;
        yield <option value='brix' key='brix'>{t('brix')}</option>;
    }
}

const onChangeUnit = (props, unit) => {
    props.changeUnit(unit);
}

const ColorCalc = (props) => {
    const { t } = useTranslation();
    return (
        <div className="container top-title">
            <div className="columns">
                <div className="column is-full">
                    <div className="title is-4">{t('color calculator')}</div>
                </div>
            </div>
            <div className="card">
                <div className="card-content">
                    <div className="columns is-multiline">

                        <div className="column is-full">
                            <div className="columns is-vcentered">
                                <div className="column has-text-right">
                                    <div>{t('original gravity')}</div>
                                </div>
                                <div className="column is-2 is-narrow">
                                    <div>
                                        <input className="input"
                                            type="number"
                                            min=".1"
                                            max="40"
                                            step=".1"
                                            value={props.gravity.original}
                                            onChange={(e) => changeOriginalGravity(props, e.target.value)}></input>
                                    </div>
                                </div>
                                <div className="column is-4 is-narrow">
                                    <div className="select" onChange={(e) => onChangeUnit(props, e.target.value)}>
                                        <select>
                                            {[...getOptions(props, t)]}
                                        </select>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div className="column is-full">
                            <div className="columns is-vcentered">
                                <div className="column has-text-right">
                                    <div>{t('boiling time')}</div>
                                </div>
                                <div className="column is-2 is-narrow">
                                    <input className="input" type="number" min="1" max="600" step="1" value={props.ibu.boiling} onChange={(e) => onBoilingChange(props, e.target.value)}></input>
                                </div>
                                <div className="column is-4 is-narrow">{t('minutes')}
                                </div>
                            </div>
                        </div>

                    </div>
                    <div className="columns">
                        <div className="column is-full"><MaltAdder /></div>
                    </div>
                    <div className="columns">
                        <div className="column has-text-right"><ColorDisplay /></div>
                    </div>
                </div>
            </div>
        </div>
    );
}

const mapStateToProps = (state) => {
    return { gravity: state.beerCalc.gravity, unit: state.beerCalc.unit, ibu: state.beerCalc.ibu };
}

export default connect(mapStateToProps, { changeUnit, originalGravity, boilingTime })(ColorCalc);