import { TFunction, useTranslation } from "react-i18next";
import { connect, ConnectedProps } from "react-redux";
import { boilingTime, changeFlameout, changeFlameoutTemp, changeUnit, changeVolume, setOriginalGravity } from '../../actions';
import { CalculatorState, Unit } from "../../types";

interface IbuPresetProps extends PropsFromRedux { }

const changeOriginalGravity = (props: IbuPresetProps, newOg: string) => {
    let og = parseFloat(newOg);
    if (isNaN(og) || og === null) {
        return;
    }
    props.setOriginalGravity({ unit: props.originalGravity.unit, amount: og });
}

const getOptions = function* (props: IbuPresetProps, t: TFunction<"translation", undefined>) {
    if (props.unit === Unit.Brix) {
        yield <option value='brix' key='brix'>{t('brix')}</option>;
        yield <option value='plato' key='plato'>{t('plato')}</option>;
    } else {
        yield <option value='plato' key='plato'>{t('plato')}</option>;
        yield <option value='brix' key='brix'>{t('brix')}</option>;
    }
}

const onChangeUnit = (props: IbuPresetProps, unit: Unit) => {
    props.changeUnit(unit);
}

const onBoilingChange = (props: IbuPresetProps, time: string) => {
    let boil = parseFloat(time);
    if (isNaN(boil) || boil === null) {
        return;
    }
    props.boilingTime(boil);

}

const onVolumeChange = (props: IbuPresetProps, newVol: string) => {
    let vol = parseFloat(newVol);
    if (isNaN(vol) || vol === null) {
        return;
    }
    props.changeVolume(vol);
}

const onFlameoutChange = (props: IbuPresetProps, newTime: string) => {
    let time = parseFloat(newTime);
    if (isNaN(time) || time === null) {
        return;
    }
    props.changeFlameout(time);
}

const onFlameoutTempChange = (props: IbuPresetProps, newTemp: string) => {
    let temp = parseFloat(newTemp);
    if (isNaN(temp) || temp === null) {
        return;
    }
    props.changeFlameoutTemp(temp);
}


const IbuPreset = (props: IbuPresetProps) => {
    const { t } = useTranslation();
    if (!props || !props.unit) {
        return <div>Loading...</div>;
    }
    return (
        <div className="flex flex-col gap-4 shadow-md p-4">
            <div className='flex flex-row gap-4 items-baseline flex-wrap'>
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
            <div className='flex flex-row gap-4 items-baseline flex-wrap'>
                <div>{t('boiling time')}</div>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="1" max="600" step="1" value={props.ibu.boiling} onChange={(e) => onBoilingChange(props, e.target.value)}></input>
                <div>{t('minutes')}</div>
            </div>
            <div className='flex flex-row gap-4 items-baseline flex-wrap'>
                <div>{t('volume')}</div>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="1" max="500" step="1" value={props.ibu.volume} onChange={(e) => onVolumeChange(props, e.target.value)}></input>
                <div>{t('liter')}</div>
            </div>
            <div className='flex flex-row gap-4 items-baseline flex-wrap'>
                <div>{t('flameout time')}</div>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="0" max="90" step="1" value={props.ibu.flameout} onChange={(e) => onFlameoutChange(props, e.target.value)}></input>
                <div>{t('minutes')}</div>
            </div>
            <div className='flex flex-row gap-4 items-baseline flex-wrap'>
                <div>{t('flameout temp')}</div>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="20" max="100" step="1" value={props.ibu.flameoutTemp} onChange={(e) => onFlameoutTempChange(props, e.target.value)}></input>
                <div>°C</div>
            </div>
        </div>
    );
}

const mapStateToProps = (rootState: any) => {
    return rootState.beerCalc as CalculatorState;
}

const connector = connect(mapStateToProps, { setOriginalGravity, changeUnit, changeFlameout, changeFlameoutTemp, boilingTime, changeVolume });
type PropsFromRedux = ConnectedProps<typeof connector>;

export default connector(IbuPreset);