import { TFunction, useTranslation } from 'react-i18next';
import { connect, ConnectedProps } from "react-redux";
import { boilingTime, changeUnit, setOriginalGravity } from "../../actions";
import { CalculatorState, Gravity, IbuState, Unit } from '../../types';
import ColorDisplay from './ColorDisplay';
import MaltAdder from './MaltAdder';

interface ColorCalcProps extends PropsFromRedux {
    originalGravity: Gravity;
    unit: Unit;
    ibu: IbuState;
}

const changeOriginalGravity = (props: ColorCalcProps, newOg: string) => {
    let og = parseFloat(newOg);
    if (isNaN(og) || og === null) {
        return;
    }
    props.setOriginalGravity({ unit: props.originalGravity.unit, amount: og });
}

const onBoilingChange = (props: ColorCalcProps, time: string) => {
    let boil = parseFloat(time);
    if (isNaN(boil) || boil === null) {
        return;
    }
    props.boilingTime(boil);
}

const getOptions = function* (props: ColorCalcProps, t: TFunction<"translation", undefined>) {
    if (props.unit === Unit.Brix) {
        yield <option value='brix' key='brix'>{t('brix')}</option>;
        yield <option value='plato' key='plato'>{t('plato')}</option>;
    } else {
        yield <option value='plato' key='plato'>{t('plato')}</option>;
        yield <option value='brix' key='brix'>{t('brix')}</option>;
    }
}

const onChangeUnit = (props: ColorCalcProps, unit: Unit) => {
    props.changeUnit(unit);
}

const ColorCalc: React.FC<ColorCalcProps> = (props: ColorCalcProps) => {
    const { t } = useTranslation();
    return (
        <div className="flex flex-col gap-4 dark:text-gray-400">
            <div className="text-2xl my-3">{t('color calculator')}</div>
            <div className="grid grid-cols-12 gap-3 shadow-md p-4 items-baseline">
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('original gravity')}</div>
                <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                    <input
                        type="number"
                        min=".1"
                        max="40"
                        step=".1"
                        className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                        value={props.originalGravity.amount}
                        onChange={(e) => changeOriginalGravity(props, e.target.value)}></input>
                </div>
                <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">
                    <select className="p-1 appearance-none rounded-none border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" onChange={(e) => onChangeUnit(props, e.target.value as Unit)}>
                        {[...getOptions(props, t)]}
                    </select>
                </div>
                <div className="2xl:col-span-3 xl:col-span-4 lg:col-span-5 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right">{t('boiling time')}</div>
                <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 xs:col-span-8 col-span-8">
                    <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="1" max="600" step="1" value={props.ibu.boiling} onChange={(e) => onBoilingChange(props, e.target.value)}></input>
                </div>
                <div className="2xl:col-span-7 xl:col-span-5 lg:col-span-4 md:col-span-2 sm:col-span-6 xs:col-span-4 col-span-4">{t('minutes')}
                </div>
            </div>
            <div className=""><MaltAdder /></div>
            <div className=""><ColorDisplay /></div>
        </div>
    );
}

const mapStateToProps = (rootState: any) => {
    let state: CalculatorState = rootState.beerCalc;
    return { originalGravity: state.originalGravity, unit: state.unit, ibu: state.ibu };
}

const connector = connect(mapStateToProps, { changeUnit, setOriginalGravity, boilingTime });
type PropsFromRedux = ConnectedProps<typeof connector>;

export default connector(ColorCalc);