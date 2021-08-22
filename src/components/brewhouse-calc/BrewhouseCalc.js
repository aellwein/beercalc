import React from "react";
import { useTranslation } from "react-i18next";
import { connect } from "react-redux";
import { changeUnit, changeUnit2, changeVolumeMeasuredAt, changeVolumeMeasuredAt2, changeWortVolume, changeWortVolume2, originalGravity, originalGravity2, setGrainMassAndUnit, changeBrewhouseEfficiency } from '../../actions';
import { convertUnits } from "../units/calculations";

const changeGrainMass = (props, val) => {
    let mass = parseFloat(val);
    if (isNaN(mass) || mass === null) {
        return;
    }
    props.setGrainMassAndUnit(mass, props.brewhouse.grainMassUnit);
}

const changeGrainMassUnit = (props, unit) => {
    props.setGrainMassAndUnit(props.brewhouse.grainMass, unit);
}

const changeOriginalGravity = (props, newOg) => {
    let og = parseFloat(newOg);
    if (isNaN(og) || og === null) {
        return;
    }
    props.originalGravity(og);
}

const changeOriginalGravity2 = (props, newOg) => {
    let og = parseFloat(newOg);
    if (isNaN(og) || og === null) {
        return;
    }
    props.originalGravity2(og);
}

const onChangeBrewhouseEfficiency = (props, val) => {
    let be = parseFloat(val);
    if (isNaN(be) || be === null) {
        return;
    }
    props.changeBrewhouseEfficiency(be);
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

const getOptions2 = function* (props, t) {
    if (props.brewhouse.grainMassFromBrewhouse.originalGravityUnit === 'brix') {
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

const onChangeUnit2 = (props, unit) => {
    props.changeUnit2(unit);
}

const onChangeWortVolume = (props, val) => {
    let wortVol = parseFloat(val);
    if (isNaN(wortVol) || wortVol === null) {
        return;
    }
    props.changeWortVolume(wortVol);
}

const onChangeWortVolume2 = (props, val) => {
    let wortVol = parseFloat(val);
    if (isNaN(wortVol) || wortVol === null) {
        return;
    }
    props.changeWortVolume2(wortVol);
}


const onChangeVolumeMeasuredAt = (props, val) => {
    props.changeVolumeMeasuredAt(val);
}

const onChangeVolumeMeasuredAt2 = (props, val) => {
    props.changeVolumeMeasuredAt2(val);
}

const getBrewhouseEfficiency = (props) => {
    const og = convertUnits(props.gravity.original, props.unit);
    const grainMassKg = (props.brewhouse.grainMassUnit === 'g') ? props.brewhouse.grainMass / 1000 : props.brewhouse.grainMass;
    const tempFactor = (props.brewhouse.volumeMeasuredAt === '100') ? 0.96 : 1;
    const brewhouseEfficiency = (props.brewhouse.wortVolume * og.sg * (og.plato / 100) * tempFactor / grainMassKg) * 100;
    return brewhouseEfficiency.toFixed(1);
}

const getGrainMassFromBrewhouse = (props) => {
    const og = convertUnits(props.brewhouse.grainMassFromBrewhouse.originalGravity, props.brewhouse.grainMassFromBrewhouse.originalGravityUnit);
    const tempFactor = (props.brewhouse.grainMassFromBrewhouse.volumeMeasuredAt === '100') ? 0.96 : 1;
    const grainMassKg = (props.brewhouse.grainMassFromBrewhouse.wortVolume * og.sg * (og.plato / 100) * tempFactor / props.brewhouse.grainMassFromBrewhouse.brewhouseEfficiency) * 100;
    return grainMassKg.toFixed(1);
}

const BrewhouseEfficiencyCalc = (props) => {
    const { t } = useTranslation();
    return (
        <div className="flex flex-col gap-4 dark:text-gray-400">
            <div className="text-2xl my-3">{t('brewhouse calculator')}</div>
            <div className="grid grid-cols-12 gap-3 shadow-md p-4 items-baseline">
                <div className="col-span-12 text-xl">{t('brewhouse efficiency')}</div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('grain mass')}</div>
                <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                    <input
                        type="number"
                        min=".1"
                        max="10000"
                        step=".1"
                        className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        value={props.brewhouse.grainMass}
                        onChange={e => changeGrainMass(props, e.target.value)}
                    ></input>
                </div>
                <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">
                    <select className="p-1 border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" onChange={e => changeGrainMassUnit(props, e.target.value)}>
                        <option value="kg">{t('kg')}</option>
                        <option value="g">{t('g')}</option>
                    </select>
                </div>
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
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('wort volume')}</div>
                <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                    <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        type="number"
                        min=".1"
                        max="40"
                        step=".1"
                        value={props.brewhouse.wortVolume}
                        onChange={(e) => onChangeWortVolume(props, e.target.value)}></input>
                </div>
                <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">{t('liter')}</div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('volume measured at')}</div>
                <div className="2xl:col-span-1 xl:col-span-1 lg:col-span-2 md:col-span-3 col-span-12 xs:col-span-12">
                    <label>
                        <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="100_C" name="hundred" checked={props.brewhouse.volumeMeasuredAt === '100'} onChange={() => onChangeVolumeMeasuredAt(props, '100')} />
                        &nbsp;&nbsp;100 °C
                    </label>
                </div>
                <div className="2xl:col-span-8 xl:col-span-7 lg:col-span-5 md:col-span-3 col-span-12 xs:col-span-12">
                    <label>
                        <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="20_C" name="twenty" checked={props.brewhouse.volumeMeasuredAt === '20'} onChange={() => onChangeVolumeMeasuredAt(props, '20')} />
                        &nbsp;&nbsp;20 °C
                    </label>
                </div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12"><strong>{t('brewhouse efficiency')}:&nbsp;{getBrewhouseEfficiency(props)}&nbsp;%</strong></div>
            </div>


            <div className="grid grid-cols-12 gap-3 shadow-md p-4 items-baseline">
                <div className="col-span-12 text-xl">{t('grain mass relative to brewhouse efficiency')}</div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('brewhouse efficiency')}</div>
                <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                    <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        type="number"
                        min=".1"
                        max="100"
                        step=".1"
                        value={props.brewhouse.grainMassFromBrewhouse.brewhouseEfficiency}
                        onChange={(e) => onChangeBrewhouseEfficiency(props, e.target.value)}></input>
                </div>
                <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">%</div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('original gravity')}</div>
                <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                    <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        type="number"
                        min=".1"
                        max="40"
                        step=".1"
                        value={props.brewhouse.grainMassFromBrewhouse.originalGravity}
                        onChange={(e) => changeOriginalGravity2(props, e.target.value)}></input>
                </div>
                <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">
                    <select className="p-1 border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" onChange={(e) => onChangeUnit2(props, e.target.value)}>
                        {[...getOptions2(props, t)]}
                    </select>
                </div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('wort volume')}</div>
                <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                    <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        type="number"
                        min=".1"
                        max="40"
                        step=".1"
                        value={props.brewhouse.grainMassFromBrewhouse.wortVolume}
                        onChange={(e) => onChangeWortVolume2(props, e.target.value)}></input>
                </div>
                <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">{t('liter')}</div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('volume measured at')}</div>
                <div className="2xl:col-span-1 xl:col-span-1 lg:col-span-2 md:col-span-3 col-span-12 xs:col-span-12">
                    <label>
                        <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="2_100_C" name="2_hundred" checked={props.brewhouse.grainMassFromBrewhouse.volumeMeasuredAt === '100'} onChange={() => onChangeVolumeMeasuredAt2(props, '100')} />
                        &nbsp;&nbsp;100 °C
                    </label>
                </div>
                <div className="2xl:col-span-8 xl:col-span-7 lg:col-span-5 md:col-span-3 col-span-12 xs:col-span-12">
                    <label>
                        <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="2_20_C" name="2_twenty" checked={props.brewhouse.grainMassFromBrewhouse.volumeMeasuredAt === '20'} onChange={() => onChangeVolumeMeasuredAt2(props, '20')} />
                        &nbsp;&nbsp;20 °C
                    </label>
                </div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12"><strong>{t('grain mass')}:&nbsp;{getGrainMassFromBrewhouse(props)}&nbsp;{t('kg')}</strong></div>
            </div>
        </div>
    );
}

const mapStateToProps = (state, _) => {
    let brewhouse = state.beerCalc.brewhouse;
    if ((typeof (state.beerCalc.brewhouse.grainMass) === 'undefined') || (typeof (state.beerCalc.brewhouse.grainMassUnit) === 'undefined')) {
        if ((typeof (state.beerCalc.grain.malt) !== 'undefined') && state.beerCalc.grain.malt.length > 0) {
            for (let i = 0; i < state.beerCalc.grain.malt.length; i++) {
                if (state.beerCalc.grain.malt[i].massUnit === 'g') {
                    brewhouse.grainMass += 0.001 * state.beerCalc.grain.malt[i].mass;
                } else {
                    brewhouse.grainMass += state.beerCalc.grain.malt[i].mass;
                }
            }
        } else {
            brewhouse.grainMass = 5;
            brewhouse.grainMassUnit = 'kg';
        }
    }
    if ((typeof (brewhouse.wortVolume) === 'undefined')) {
        brewhouse.wortVolume = 28;
    }
    return { ...state.beerCalc, brewhouse };
}

export default connect(mapStateToProps, {
    setGrainMassAndUnit, changeUnit, changeUnit2, originalGravity, originalGravity2,
    changeWortVolume, changeVolumeMeasuredAt, changeWortVolume2, changeVolumeMeasuredAt2, changeBrewhouseEfficiency
})(BrewhouseEfficiencyCalc);