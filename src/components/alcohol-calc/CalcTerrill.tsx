import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { CalculatorState, Gravity, Unit } from '../../types';
import { alcoholTerrillFormula } from '../units/calculations';

interface TerrillCalcProps {
    originalGravity: Gravity;
    finalGravity: Gravity;
    unit: Unit;
}

const calcNotPossible = (t: TFunction<"translation", undefined>): JSX.Element => {
    return (
        <div className="flex flex-wrap flex-col shadow-md p-4 gap-2 flex-grow items-center">
            <div className="text-center flex-grow text-red-400 table"><span className="table-cell align-middle">{t('calculation not possible')}</span></div>
        </div>
    )
}

const calcTerrill = (props: TerrillCalcProps, t: TFunction<"translation", undefined>): JSX.Element => {

    if (props.originalGravity.amount <= props.finalGravity.amount) {
        return calcNotPossible(t);
    }

    const calc = alcoholTerrillFormula(props.originalGravity, props.finalGravity);
    if (Number.parseFloat(calc.apparentExtract.toFixed(1)) <= 0.0 || Number.parseFloat(calc.apparentAttenuation.toFixed(1)) >= 100.0) {
        return calcNotPossible(t);
    }

    return (
        <div className="flex flex-wrap flex-col shadow-md p-4 gap-2 flex-grow">
            <div className="text-2xl text-center my-3">{t('terrill equation')}</div>
            <div className="flex flex-row flex-wrap gap-2 hover:border-b-1 hover:border-neutral-700 dark:hover:border-neutral-500">
                <div className="text-left flex-grow">{t('apparent extract')}:</div>
                <div className="text-right">{calc.apparentExtract.toFixed(1)} °P</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2 hover:border-b-1 hover:border-neutral-700 dark:hover:border-neutral-500">
                <div className="text-left flex-grow">{t('real extract')}:</div>
                <div className="text-right">{calc.realExtract.toFixed(1)} %</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2 hover:border-b-1 hover:border-neutral-700 dark:hover:border-neutral-500">
                <div className="text-left flex-grow">{t('apparent attenuation')}:</div>
                <div className="text-right">{calc.apparentAttenuation.toFixed(1)} %</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2 hover:border-b-1 hover:border-neutral-700 dark:hover:border-neutral-500">
                <div className="text-left flex-grow">{t('real attenuation')}:</div>
                <div className="text-right">{calc.realAttenuation.toFixed(1)} %</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2 hover:border-b-1 hover:border-neutral-700 dark:hover:border-neutral-500">
                <div className="text-left flex-grow">{t('alcohol by weight')}:</div>
                <div className="text-right">{calc.alcByWeight.toFixed(1)} %</div>
            </div>
            <div className="flex flex-row flex-wrap gap-2 hover:border-b-1 hover:border-neutral-700 dark:hover:border-neutral-500">
                <div className="text-left flex-grow">{t('alcohol by volume')}:</div>
                <div className="text-right">{calc.alcByVolume.toFixed(1)} %</div>
            </div>
        </div>
    );
}

const CalcTerrill = (props: TerrillCalcProps) => {
    const { t } = useTranslation();

    if (!props.unit || !props.originalGravity || !props.finalGravity) {
        return <div></div>;
    }
    if (props.unit !== Unit.Brix) {
        // the Terrill formula doesn't work for extracts taken in Plato
        // (refractometer only)
        return <div></div>;
    }
    return calcTerrill(props, t);
}


const mapStateToProps = (rootState: any): TerrillCalcProps => {
    const state: CalculatorState = rootState.beerCalc;
    return {
        unit: state.unit,
        originalGravity: state.originalGravity,
        finalGravity: state.finalGravity,
    };
}

export default connect(mapStateToProps, {})(CalcTerrill);