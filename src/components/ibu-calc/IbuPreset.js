import React from "react";
import { useTranslation } from "react-i18next";
import { connect } from "react-redux";
import { boilingTime, changeFlameout, changeFlameoutTemp, changeUnit, changeVolume, originalGravity } from '../../actions';

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

const onVolumeChange = (props, newVol) => {
    let vol = parseFloat(newVol);
    if (isNaN(vol)) {
        return;
    }
    if (vol < 1) {
        vol = 1;
    }
    if (vol > 500) {
        vol = 500;
    }
    if (vol !== null) {
        props.changeVolume(vol);
    }
}

const onFlameoutChange = (props, newTime) => {
    let time = parseFloat(newTime);
    if (isNaN(time)) {
        return;
    }
    if (time < 0) {
        time = 0;
    }
    if (time > 90) {
        time = 90;
    }
    if (time !== null) {
        props.changeFlameout(time);
    }
}

const onFlameoutTempChange = (props, newTemp) => {
    let temp = parseFloat(newTemp);
    if (isNaN(temp)) {
        return;
    }
    if (temp < 20) {
        temp = 20;
    }
    if (temp > 100) {
        temp = 100;
    }
    if (temp !== null) {
        props.changeFlameoutTemp(temp);
    }
}


const IbuPreset = (props) => {
    const { t } = useTranslation();
    if (!props || !props.unit) {
        return <div>Loading...</div>;
    }
    return (
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
                    <div className="column is-full">
                        <div className="columns is-vcentered">
                            <div className="column has-text-right">
                                <div>{t('volume')}</div>
                            </div>
                            <div className="column is-2 is-narrow">
                                <input className="input" type="number" min="1" max="500" step="1" value={props.ibu.volume} onChange={(e) => onVolumeChange(props, e.target.value)}></input>
                            </div>
                            <div className="column is-4 is-narrow">{t('liter')}
                            </div>
                        </div>
                    </div>
                </div>
                <div className="column is-full">
                    <div className="columns is-vcentered">
                        <div className="column has-text-right">
                            <div>{t('flameout time')}</div>
                        </div>
                        <div className="column is-2 is-narrow">
                            <input className="input" type="number" min="0" max="90" step="1" value={props.ibu.flameout} onChange={(e) => onFlameoutChange(props, e.target.value)}></input>
                        </div>
                        <div className="column is-4 is-narrow">{t('minutes')}
                        </div>
                    </div>
                </div>
                <div className="column is-full">
                    <div className="columns is-vcentered">
                        <div className="column has-text-right">
                            <div>{t('flameout temp')}</div>
                        </div>
                        <div className="column is-2 is-narrow">
                            <input className="input" type="number" min="20" max="100" step="1" value={props.ibu.flameoutTemp} onChange={(e) => onFlameoutTempChange(props, e.target.value)}></input>
                        </div>
                        <div className="column is-4 is-narrow">°C
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

const mapStateToProps = (state, _) => {
    return state.beerCalc;
}

export default connect(mapStateToProps, { originalGravity, changeUnit, changeFlameout, changeFlameoutTemp, boilingTime, changeVolume })(IbuPreset);