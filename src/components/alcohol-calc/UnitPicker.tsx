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

const UnitPicker: React.FC<UnitPickerProps> = (props: UnitPickerProps) => {
    const { t } = useTranslation();
    if (!props.unit) {
        return <div></div>;
    }
    return (
        <div className="grid grid-cols-12 gap-3 shadow-md p-4">
            <span className="2xl:col-span-3 xl:col-span-3 lg:col-span-4 md:col-span-5 sm:col-span-6 col-span-12 xs:col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-left xs:text-left text-left">{t('gravity unit')}</span>
            <div className="2xl:col-span-1 xl:col-span-1 lg:col-span-2 md:col-span-3 col-span-12 xs:col-span-12">
                <label>
                    <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="brix" name="unit" checked={props.unit === Unit.Brix} onChange={e => onChange(props, e.target.id as Unit)} />
                    &nbsp;&nbsp;{t('brix')}
                </label>
            </div>
            <div className="2xl:col-span-1 xl:col-span-1 lg:col-span-2 md:col-span-3 col-span-12 xs:col-span-12">
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