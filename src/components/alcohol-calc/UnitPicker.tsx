import { useTranslation } from 'react-i18next';
import { connect, ConnectedProps } from 'react-redux';
import { changeUnit } from '../../actions';
import { CalculatorState, Unit } from '../../types';

const onChange = (props: UnitPickerProps, unit: Unit) => {
  props.changeUnit(unit);
}

interface UnitPickerProps extends PropsFromRedux {
  unit: Unit
}

const UnitPicker = (props: UnitPickerProps) => {
  const { t } = useTranslation();
  if (!props.unit) {
    return <div></div>;
  }
  return (
    <div className="flex flex-row gap-4 shadow-md p-4">
      <span>{t('gravity unit')}</span>
      <div>
        <label>
          <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="brix" name="unit" checked={props.unit === Unit.Brix} onChange={e => onChange(props, e.target.id as Unit)} />
          &nbsp;&nbsp;{t('brix')}
        </label>
      </div>
      <div>
        <label>
          <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="plato" name="unit" checked={props.unit === Unit.Plato} onChange={e => onChange(props, e.target.id as Unit)} />
          &nbsp;&nbsp;{t('plato')}
        </label>
      </div>
    </div>
  );
}

const mapStateToProps = (rootState: any) => {
  const state: CalculatorState = rootState.beerCalc;
  return { unit: state.unit };
}
type PropsFromRedux = ConnectedProps<typeof connector>;
const connector = connect(mapStateToProps, { changeUnit });
export default connector(UnitPicker);
