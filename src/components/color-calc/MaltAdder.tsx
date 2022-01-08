import { TFunction, useTranslation } from 'react-i18next';
import { connect, ConnectedProps } from 'react-redux';
import { newMaltAddition, removeMaltAddition, changeMaltMass, changeMaltMassUnit, changeMaltColor } from '../../actions';
import { CalculatorState, ColorCalcState, MassUnit } from '../../types';

interface MaltAdderProps extends PropsFromRedux {
    grain: ColorCalcState;
}

const onAddMalt = (props: MaltAdderProps) => {
    props.newMaltAddition();
}

const onChangeMassUnit = (props: MaltAdderProps, idx: number, val: MassUnit) => {
    props.changeMaltMassUnit(idx, val);
}

const onChangeMass = (props: MaltAdderProps, idx: number, val: string) => {
    let a = parseFloat(val);
    if (isNaN(a) || a === null) {
        return;
    }
    props.changeMaltMass(idx, a);

}

const onChangeEBC = (props: MaltAdderProps, idx: number, color: string) => {
    let a = parseInt(color);
    if (isNaN(a) || a === null) {
        return;
    }
    props.changeMaltColor(idx, a);
}

const onRemoveGrainClick = (props: MaltAdderProps, idx: number) => {
    props.removeMaltAddition(idx);
}

const grainAdditions = function* (props: MaltAdderProps, t: TFunction<"translation", undefined>) {
    for (let i = 0; i < props.grain.malt.length; i++) {
        let a = props.grain.malt[i];
        yield (
            <div className='flex flex-row gap-4 items-baseline flex-wrap' key={"grainAdd-" + i}>
                <div>{t('malt') + ' #' + (i + 1)}</div>
                <button className="py-2 px-2 bg-red-300 text-white hover:bg-red-600 p-1" onClick={() => onRemoveGrainClick(props, i)}>X</button>
            </div>
        );
        yield (
            <div className='flex flex-row gap-4 items-baseline flex-wrap' key={"mass-" + i}>
                <div>{t('mass')}</div>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="0" max="500" step=".1" value={a.amount} onChange={(e) => onChangeMass(props, i, e.target.value)}></input>
            </div>
        );
        yield (
            <div className='flex flex-row gap-4 items-baseline flex-wrap' key={"unit-" + i}>
                <div>{t('mass unit')}</div>
                <select className="p-1 appearance-none rounded-none bg-white border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" defaultValue="kg" onChange={(e) => onChangeMassUnit(props, i, e.target.value as MassUnit)}>
                    <option value="kg">{t('kg')}</option>
                    <option value="g">{t('g')}</option>
                </select>
            </div>
        );
        yield (
            <div className='flex flex-row gap-4 items-baseline flex-wrap' key={"color-" + i}>
                <div>{t('color ebc')}</div>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="1" max="10000" step="1" value={a.color} onChange={(e) => onChangeEBC(props, i, e.target.value)}></input>
            </div>
        );
    }
}

const MaltAdder: React.FC<MaltAdderProps> = (props: MaltAdderProps) => {
    const { t } = useTranslation();
    return (
        <div className="flex flex-col gap-4 shadow-md p-4">
            {[...grainAdditions(props, t)]}
            <div className='mr-auto' key={"add-button"}>
                <button className="py-2 px-4 bg-yellow-500 dark:bg-yellow-700 text-white dark:text-gray-100 rounded-lg shadow-md hover:bg-yellow-700 dark:hover:bg-yellow-400 dark:hover:text-gray-800" onClick={() => onAddMalt(props)}>{t('add malt')}</button>
            </div>
        </div>
    );
}

const mapStateToProps = (rootState: any) => {
    const state: CalculatorState = rootState.beerCalc;
    return { grain: state.grain };
}

const connector = connect(mapStateToProps, { newMaltAddition, removeMaltAddition, changeMaltMass, changeMaltMassUnit, changeMaltColor });
type PropsFromRedux = ConnectedProps<typeof connector>;

export default connector(MaltAdder);