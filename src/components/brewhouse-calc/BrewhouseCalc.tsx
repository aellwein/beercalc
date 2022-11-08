import { TFunction } from 'i18next';
import { useTranslation } from "react-i18next";
import { connect, ConnectedProps } from "react-redux";
import { changeBrewhouseEfficiency, changeUnit, changeUnit2, changeVolumeMeasuredAt, changeVolumeMeasuredAt2, changeWortVolume, changeWortVolume2, setGrainMassAndUnit, setOriginalGravity, setOriginalGravity2 } from '../../actions';
import { BrewhouseEfficiencyState, CalculatorState, Gravity, MassUnit, Unit, VolumeMeasuredAt } from "../../types";
import { convertUnits } from "../units/calculations";

interface BrewhouseCalcProps extends PropsFromRedux {
    unit: Unit;
    originalGravity: Gravity,
    brewhouse: BrewhouseEfficiencyState
}

const changeGrainMass = (props: BrewhouseCalcProps, val: string) => {
    let mass = parseFloat(val);
    if (isNaN(mass) || mass === null) {
        return;
    }
    if (props.brewhouse.grainMassUnit) {
        props.setGrainMassAndUnit(mass, props.brewhouse.grainMassUnit);
    }
}

const changeGrainMassUnit = (props: BrewhouseCalcProps, unit: MassUnit) => {
    if (props.brewhouse.grainMass) {
        props.setGrainMassAndUnit(props.brewhouse.grainMass, unit);
    }
}

const changeOriginalGravity = (props: BrewhouseCalcProps, newOg: string) => {
    let og = parseFloat(newOg);
    if (isNaN(og) || og === null) {
        return;
    }
    props.setOriginalGravity({ unit: props.unit, amount: og });
}

const changeOriginalGravity2 = (props: BrewhouseCalcProps, newOg: string) => {
    let og = parseFloat(newOg);
    if (isNaN(og) || og === null) {
        return;
    }
    props.setOriginalGravity2({ unit: props.unit, amount: og });
}

const onChangeBrewhouseEfficiency = (props: BrewhouseCalcProps, val: string) => {
    let be = parseFloat(val);
    if (isNaN(be) || be === null) {
        return;
    }
    props.changeBrewhouseEfficiency(be);
}

const getOptions = function* (props: BrewhouseCalcProps, t: TFunction<"translation", undefined>): Generator<JSX.Element, void, undefined> {
    if (props.unit === Unit.Brix) {
        yield <option value='brix' key='brix'>{t('brix')}</option>;
        yield <option value='plato' key='plato'>{t('plato')}</option>;
    } else {
        yield <option value='plato' key='plato'>{t('plato')}</option>;
        yield <option value='brix' key='brix'>{t('brix')}</option>;
    }
}

const getOptions2 = function* (props: BrewhouseCalcProps, t: TFunction<"translation", undefined>): Generator<JSX.Element, void, undefined> {
    if (props.brewhouse.grainMassFromBrewhouse.originalGravity.unit === Unit.Brix) {
        yield <option value='brix' key='brix'>{t('brix')}</option>;
        yield <option value='plato' key='plato'>{t('plato')}</option>;
    } else {
        yield <option value='plato' key='plato'>{t('plato')}</option>;
        yield <option value='brix' key='brix'>{t('brix')}</option>;
    }
}

const onChangeUnit = (props: BrewhouseCalcProps, unit: Unit) => {
    props.changeUnit(unit);
}

const onChangeUnit2 = (props: BrewhouseCalcProps, unit: Unit) => {
    props.changeUnit2(unit);
}

const onChangeWortVolume = (props: BrewhouseCalcProps, val: string) => {
    let wortVol = parseFloat(val);
    if (isNaN(wortVol) || wortVol === null) {
        return;
    }
    props.changeWortVolume(wortVol);
}

const onChangeWortVolume2 = (props: BrewhouseCalcProps, val: string) => {
    let wortVol = parseFloat(val);
    if (isNaN(wortVol) || wortVol === null) {
        return;
    }
    props.changeWortVolume2(wortVol);
}


const onChangeVolumeMeasuredAt = (props: BrewhouseCalcProps, val: VolumeMeasuredAt) => {
    props.changeVolumeMeasuredAt(val);
}

const onChangeVolumeMeasuredAt2 = (props: BrewhouseCalcProps, val: VolumeMeasuredAt) => {
    props.changeVolumeMeasuredAt2(val);
}

const getBrewhouseEfficiency = (props: BrewhouseCalcProps): number => {
    const og = convertUnits(props.originalGravity.amount, props.originalGravity.unit);
    const grainMassKg = (props.brewhouse.grainMassUnit === MassUnit.Gram) ? props.brewhouse.grainMass / 1000 : props.brewhouse.grainMass;
    const tempFactor = (props.brewhouse.volumeMeasuredAt === VolumeMeasuredAt.Hundred) ? 0.96 : 1;
    const brewhouseEfficiency = (props.brewhouse.wortVolume * og.sg * (og.plato / 100) * tempFactor / grainMassKg) * 100;
    return Number.parseFloat(brewhouseEfficiency.toFixed(1));
}

const getGrainMassFromBrewhouse = (props: BrewhouseCalcProps): number => {
    const og = convertUnits(props.brewhouse.grainMassFromBrewhouse.originalGravity.amount, props.brewhouse.grainMassFromBrewhouse.originalGravity.unit);
    const tempFactor = (props.brewhouse.grainMassFromBrewhouse.volumeMeasuredAt === VolumeMeasuredAt.Hundred) ? 0.96 : 1;
    const grainMassKg = (props.brewhouse.grainMassFromBrewhouse.wortVolume * og.sg * (og.plato / 100) * tempFactor / props.brewhouse.grainMassFromBrewhouse.brewhouseEfficiency) * 100;
    return Number.parseFloat(grainMassKg.toFixed(1));
}

const BrewhouseEfficiencyCalc = (props: BrewhouseCalcProps) => {
    const { t } = useTranslation();
    return (
        <div className="flex flex-col gap-4 dark:text-gray-400">
            <div className="text-2xl my-3">{t('brewhouse calculator')}</div>
            <div className='flex flex-row gap-4 flex-wrap'>
                <div className="flex flex-col gap-4 shadow-md p-4 items-baseline grow">
                    <div className="text-xl">{t('brewhouse efficiency')}</div>
                    <div className="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t('grain mass')}</div>
                        <input
                            type="number"
                            min=".1"
                            max="10000"
                            step=".1"
                            className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                            value={props.brewhouse.grainMass}
                            onChange={e => changeGrainMass(props, e.target.value)}
                        ></input>
                        <select className="p-1 appearance-none rounded-none bg-white border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" onChange={e => changeGrainMassUnit(props, e.target.value as MassUnit)}>
                            <option value="kg">{t('kg')}</option>
                            <option value="g">{t('g')}</option>
                        </select>
                    </div>
                    <div className="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t('original gravity')}</div>
                        <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                            type="number"
                            min=".1"
                            max="40"
                            step=".1"
                            value={props.originalGravity.amount}
                            onChange={(e) => changeOriginalGravity(props, e.target.value)}></input>
                        <select className="p-1 appearance-none rounded-none bg-white border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" onChange={(e) => onChangeUnit(props, e.target.value as Unit)}>
                            {[...getOptions(props, t)]}
                        </select>
                    </div>
                    <div className="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t('wort volume')}</div>
                        <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                            type="number"
                            min=".1"
                            max="40"
                            step=".1"
                            value={props.brewhouse.wortVolume}
                            onChange={(e) => onChangeWortVolume(props, e.target.value)}></input>
                        <div>{t('liter')}</div>
                    </div>
                    <div className="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t('volume measured at')}</div>
                        <label>
                            <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="100_C" name="hundred" checked={props.brewhouse.volumeMeasuredAt === VolumeMeasuredAt.Hundred} onChange={() => onChangeVolumeMeasuredAt(props, VolumeMeasuredAt.Hundred)} />
                            &nbsp;&nbsp;100 °C
                        </label>
                        <label>
                            <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="20_C" name="twenty" checked={props.brewhouse.volumeMeasuredAt === VolumeMeasuredAt.Twenty} onChange={() => onChangeVolumeMeasuredAt(props, VolumeMeasuredAt.Twenty)} />
                            &nbsp;&nbsp;20 °C
                        </label>
                    </div>
                    <div><strong>{t('brewhouse efficiency')}:&nbsp;{getBrewhouseEfficiency(props)}&nbsp;%</strong></div>
                </div>


                <div className="flex flex-col gap-4 shadow-md p-4 dark:text-gray-400 grow">
                    <div className="text-xl">{t('grain mass relative to brewhouse efficiency')}</div>
                    <div className="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t('brewhouse efficiency')}</div>
                        <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                            type="number"
                            min=".1"
                            max="100"
                            step=".1"
                            value={props.brewhouse.grainMassFromBrewhouse.brewhouseEfficiency}
                            onChange={(e) => onChangeBrewhouseEfficiency(props, e.target.value)}></input>
                        <div >%</div>
                    </div>
                    <div className="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t('original gravity')}</div>
                        <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                            type="number"
                            min=".1"
                            max="40"
                            step=".1"
                            value={props.brewhouse.grainMassFromBrewhouse.originalGravity.amount}
                            onChange={(e) => changeOriginalGravity2(props, e.target.value)}></input>
                        <select className="p-1 appearance-none rounded-none bg-white border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" onChange={(e) => onChangeUnit2(props, e.target.value as Unit)}>
                            {[...getOptions2(props, t)]}
                        </select>
                    </div>
                    <div className="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t('wort volume')}</div>
                        <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                            type="number"
                            min=".1"
                            max="40"
                            step=".1"
                            value={props.brewhouse.grainMassFromBrewhouse.wortVolume}
                            onChange={(e) => onChangeWortVolume2(props, e.target.value)}></input>
                        <div>{t('liter')}</div>
                    </div>
                    <div className="flex flex-row gap-4 items-baseline flex-wrap">
                        <div>{t('volume measured at')}</div>
                        <label>
                            <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="2_100_C" name="2_hundred" checked={props.brewhouse.grainMassFromBrewhouse.volumeMeasuredAt === VolumeMeasuredAt.Hundred} onChange={() => onChangeVolumeMeasuredAt2(props, VolumeMeasuredAt.Hundred)} />
                            &nbsp;&nbsp;100 °C
                        </label>
                        <label>
                            <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="2_20_C" name="2_twenty" checked={props.brewhouse.grainMassFromBrewhouse.volumeMeasuredAt === VolumeMeasuredAt.Twenty} onChange={() => onChangeVolumeMeasuredAt2(props, VolumeMeasuredAt.Twenty)} />
                            &nbsp;&nbsp;20 °C
                        </label>
                    </div>
                    <div><strong>{t('grain mass')}:&nbsp;{getGrainMassFromBrewhouse(props)}&nbsp;{t('kg')}</strong></div>
                </div>
            </div>
        </div>
    );
}

const mapStateToProps = (rootState: any) => {
    const state: CalculatorState = rootState.beerCalc;
    let brewh: BrewhouseEfficiencyState = {
        grainMass: state.brewhouse.grainMass,
        grainMassUnit: MassUnit.Kilogram,
        volumeMeasuredAt: state.brewhouse.volumeMeasuredAt,
        grainMassFromBrewhouse: state.brewhouse.grainMassFromBrewhouse,
        wortVolume: state.brewhouse.wortVolume,
    };

    if (state.grain.malt.length > 0) {
        for (let i = 0; i < state.grain.malt.length; i++) {
            if (state.grain.malt[i].massUnit === MassUnit.Gram) {
                brewh.grainMass += 0.001 * state.grain.malt[i].amount;
            } else if (state.grain.malt[i].massUnit === MassUnit.Kilogram) {
                brewh.grainMass += state.grain.malt[i].amount;
            }
        }
    }
    return {
        brewhouse: brewh,
        originalGravity: state.originalGravity,
        unit: state.unit,
    };
}

const connector = connect(mapStateToProps, {
    setGrainMassAndUnit,
    changeUnit,
    changeUnit2,
    setOriginalGravity,
    setOriginalGravity2,
    changeWortVolume,
    changeVolumeMeasuredAt,
    changeWortVolume2,
    changeVolumeMeasuredAt2,
    changeBrewhouseEfficiency
});
type PropsFromRedux = ConnectedProps<typeof connector>;

export default connector(BrewhouseEfficiencyCalc);