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
        <div className="grid grid-cols-12 gap-3 shadow-md p-4 items-baseline">
            <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('original gravity')}</div>
            <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                    type="number"
                    min=".1"
                    max="40"
                    step=".1"
                    value={props.gravity.original}
                    onChange={(e) => changeOriginalGravity(props, e.target.value)}></input>
            </div>
            <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">
                <select className="p-1 border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" onChange={(e) => onChangeUnit(props, e.target.value)}>
                    {[...getOptions(props, t)]}
                </select>
            </div>


            <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('boiling time')}</div>
            <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="1" max="600" step="1" value={props.ibu.boiling} onChange={(e) => onBoilingChange(props, e.target.value)}></input>
            </div>
            <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">{t('minutes')}</div>


            <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('volume')}</div>
            <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="1" max="500" step="1" value={props.ibu.volume} onChange={(e) => onVolumeChange(props, e.target.value)}></input>
            </div>
            <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">{t('liter')}</div>


            <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('flameout time')}</div>
            <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="0" max="90" step="1" value={props.ibu.flameout} onChange={(e) => onFlameoutChange(props, e.target.value)}></input>
            </div>
            <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">{t('minutes')}</div>


            <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('flameout temp')}</div>
            <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="20" max="100" step="1" value={props.ibu.flameoutTemp} onChange={(e) => onFlameoutTempChange(props, e.target.value)}></input>
            </div>
            <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">°C</div>
        </div>
    );
}

const mapStateToProps = (state, _) => {
    return state.beerCalc;
}

export default connect(mapStateToProps, { originalGravity, changeUnit, changeFlameout, changeFlameoutTemp, boilingTime, changeVolume })(IbuPreset);