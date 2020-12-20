import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { newMaltAddition, removeMaltAddition, changeMaltMass, changeMaltMassUnit, changeMaltColor } from '../../actions';

const onAddMalt = (props) => {
    props.newMaltAddition();
}

const onChangeMassUnit = (props, idx, val) => {
    props.changeMaltMassUnit(idx, val);
}

const onChangeMass = (props, idx, val) => {
    let a = parseFloat(val);
    if (isNaN(a)) {
        return;
    }
    if (a < .1) {
        a = .1;
    }
    if (a > 5000) {
        a = 5000;
    }
    if (a !== null) {
        props.changeMaltMass(idx, a);
    }
}

const onChangeEBC = (props, idx, color) => {
    let a = parseInt(color);
    if (isNaN(a)) {
        return;
    }
    if (a < 1) {
        a = 1;
    }
    if (a > 10000) {
        a = 10000;
    }
    if (a !== null) {
        props.changeMaltColor(idx, a);
    }
}

const onRemoveGrainClick = (props, idx) => {
    props.removeMaltAddition(idx);
}

const grainAdditions = function* (props, t) {
    for (let i = 0; i < props.grain.malt.length; i++) {
        let a = props.grain.malt[i];
        yield (
            <div className="columns is-multiline is-vcentered" key={"malt-" + i}>
                <div className="column has-text-right">{t('malt') + ' #' + (i + 1)}</div>
                <div className="column has-text-right">
                    <input className="input" type="number" min="0" max="500" step=".1" value={a.mass} onChange={(e) => onChangeMass(props, i, e.target.value)}></input>
                </div>
                <div className="column has-text-right">
                    <div className="select">
                        <select defaultValue="kg" onChange={(e) => onChangeMassUnit(props, i, e.target.value)}>
                            <option value="kg">{t('kg')}</option>
                            <option value="g">{t('g')}</option>
                        </select>
                    </div>

                </div>
                <div className="column has-text-right">
                    <input className="input" type="number" min="1" max="10000" step="1" value={a.color} onChange={(e) => onChangeEBC(props, i, e.target.value)}></input>
                </div>
                <div className="column is-1">
                    <div className="button" onClick={() => onRemoveGrainClick(props, i)}>X</div>
                </div>
            </div>
        );
    }
}


const MaltAdder = (props) => {
    const { t } = useTranslation();
    return (
        <div className="card">
            <div className="card-content">
                <div className="columns is-multiline">
                    <div className="column">&nbsp;</div>
                    <div className="column has-text-right">{t('mass')}</div>
                    <div className="column has-text-right">{t('mass unit')}</div>
                    <div className="column has-text-right">{t('color ebc')}</div>
                    <div className="column is-1">&nbsp;</div>
                </div>
                {[...grainAdditions(props, t)]}
                <div className="columns is-multiline is-vcentered">
                    <div className="column has-text-right"><div className="button is-primary" onClick={() => onAddMalt(props)}>{t('add malt')}</div></div>
                    <div className="column has-text-right"></div>
                    <div className="column has-text-right"></div>
                    <div className="column has-text-right"></div>
                </div>
            </div>
        </div>
    );
}

const mapStateToProps = (state) => {
    return { grain: state.beerCalc.grain };
}

export default connect(mapStateToProps, { newMaltAddition, removeMaltAddition, changeMaltMass, changeMaltMassUnit, changeMaltColor })(MaltAdder);