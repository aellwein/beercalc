import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { CalculatorState, Gravity, Unit } from '../../types';
import CalcStandard from './CalcStandard';
import CalcTerrill from './CalcTerrill';
import GravityPicker from './GravityPicker';
import UnitPicker from './UnitPicker';

export interface AlcoholCalcProps {
    unit: Unit;
    originalGravity: Gravity;
    finalGravity: Gravity;
}


const columns = function* (props: AlcoholCalcProps): Generator<JSX.Element, void, undefined> {
    if (props.unit === Unit.Brix) {
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12" key="calc-standard">
                <CalcStandard />
            </div>
        );
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12" key="calc-terrill">
                <CalcTerrill />
            </div>
        );
    } else {
        yield (
            <div className="col-span-12" key="calc-standard">
                <CalcStandard />
            </div>
        );
    }
}

const AlcoholCalc: React.FC<AlcoholCalcProps> = (props: AlcoholCalcProps) => {
    const { t } = useTranslation();
    return (
        <div className="flex flex-col gap-4 dark:text-gray-400">
            <div className="text-2xl my-3">{t('alcohol calculator')}</div>
            <div><UnitPicker /></div>
            <div><GravityPicker /></div>
            <div className="grid grid-cols-12 gap-4">
                {[...columns(props)]}
            </div>
        </div>
    );
}

const mapStateToProps = (rootState: any): AlcoholCalcProps => {
    const state: CalculatorState = rootState.beerCalc;
    let props: AlcoholCalcProps = {
        unit: state.unit,
        originalGravity: state.originalGravity,
        finalGravity: state.finalGravity,
    };
    return props;
}

export default connect(mapStateToProps, {})(AlcoholCalc);