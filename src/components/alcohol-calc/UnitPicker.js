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
        <div className="flex flex-row gap-4 shadow-md p-4">
            <label className="radio">
                <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="brix" name="unit" checked={isChecked(props.unit, 'brix')} onChange={e => onChange(props, e.target.id)} />
                &nbsp;&nbsp;{t('brix')}
            </label>
            <label className="radio">
                <input type="radio" className="dark:bg-gray-700 dark:text-gray-300" id="plato" name="unit" checked={isChecked(props.unit, 'plato')} onChange={e => onChange(props, e.target.id)} />
                &nbsp;&nbsp;{t('plato')}
            </label>
        </div>
    );
}

const mapStateToProps = (state) => {
    return { unit: state.beerCalc.unit };
}

export default connect(mapStateToProps, { changeUnit })(UnitPicker);