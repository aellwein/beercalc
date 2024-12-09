import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { CalculatorState, Gravity, Unit } from '../../types';
import CalcStandard from './CalcStandard';
import CalcTerrill from './CalcTerrill';
import GravityPicker from './GravityPicker';
import UnitPicker from './UnitPicker';
import { ReactElement } from 'react';

export interface AlcoholCalcProps {
  unit: Unit;
  originalGravity: Gravity;
  finalGravity: Gravity;
}


const columns = function* (props: AlcoholCalcProps): Generator<ReactElement, void, undefined> {
  if (props.unit === Unit.Brix) {
    yield (
      <CalcStandard key='calc-standard' />
    );
    yield (
      <CalcTerrill key='calc-terrill' />
    );
  } else {
    yield (
      <CalcStandard key='calc-standard' />
    );
  }
}

const AlcoholCalc = (props: AlcoholCalcProps) => {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col gap-4 dark:text-gray-400">
      <div className="text-2xl my-3">{t('alcohol calculator')}</div>
      <div><UnitPicker /></div>
      <div><GravityPicker /></div>
      <div className="flex flex-row flex-wrap gap-4">
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
