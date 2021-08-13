import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { changeUnit } from '../../actions';

const isChecked = (unit, forUnit) => {
    return unit === forUnit;
}

const onChange = (props, unit) => {
    props.changeUnit(unit);
}

const UnitPicker = (props) => {
    const { t } = useTranslation();
    if (!props.unit) {
        return <div></div>;
    }
    return (
        <div className="grid grid-cols-12 gap-3 shadow-md p-4">
            <span className="2xl:col-span-3 xl:col-span-3 lg:col-span-4 md:col-span-5 sm:col-span-6 col-span-12 xs:col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-left xs:text-left text-left">{t('gravity unit')}</span>
            <div className="2xl:col-span-1 xl:col-span-1 lg:col-span-2 md:col-span-3 col-span-12 xs:col-span-12">
                <label>
                    <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="brix" name="unit" checked={isChecked(props.unit, 'brix')} onChange={e => onChange(props, e.target.id)} />
                    &nbsp;&nbsp;{t('brix')}
                </label>
            </div>
            <div className="2xl:col-span-1 xl:col-span-1 lg:col-span-2 md:col-span-3 col-span-12 xs:col-span-12">
                <label>
                    <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="plato" name="unit" checked={isChecked(props.unit, 'plato')} onChange={e => onChange(props, e.target.id)} />
                    &nbsp;&nbsp;{t('plato')}
                </label>
            </div>
        </div>
    );
}

const mapStateToProps = (state) => {
    return { unit: state.beerCalc.unit };
}

export default connect(mapStateToProps, { changeUnit })(UnitPicker);