import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { finalGravity, originalGravity } from '../../actions';
import ShowUnits from './ShowUnits';

const onChangeOg = (props, newOg) => {
    let og = parseFloat(newOg);
    if (isNaN(og)) {
        return;
    }
    if (og < .1) {
        og = .1;
    }
    if (og > 40) {
        og = 40;
    }
    if (og !== null) {
        props.originalGravity(og);
    }
}

const onChangeFg = (props, newFg) => {
    let fg = parseFloat(newFg);
    if (isNaN(fg)) {
        return;
    }
    if (fg < .1) {
        fg = .1;
    }
    if (fg > 40) {
        fg = 40;
    }
    if (fg !== null) {
        props.finalGravity(fg);
    }
}

const GravityPicker = (props) => {
    const { t } = useTranslation();
    return (
        <div className="card">
            <div className="card-content">
                <div className="columns">
                    <div className="column is-full">
                        <span>{t('original gravity')}</span>
                        <input className="input" type="number" id="og" min=".1" max="40" step=".1" value={props.gravity.original} onChange={e => onChangeOg(props, e.target.value)}></input>
                        <ShowUnits type="og" unit={props.unit} gravity={props.gravity} />
                    </div>
                </div>
                <div className="columns">
                    <div className="column is-full">
                        <span>{t('final gravity')}</span>
                        <input className="input" type="number" id="fg" min=".1" max="40" step=".1" value={props.gravity.final} onChange={e => onChangeFg(props, e.target.value)}></input>
                        <ShowUnits type="fg" unit={props.unit} gravity={props.gravity} />
                    </div>
                </div>
            </div>
        </div >
    );
}

const mapStateToProps = (state) => {
    return { gravity: state.beerCalc.gravity, unit: state.beerCalc.unit };
}

export default connect(mapStateToProps, { originalGravity, finalGravity })(GravityPicker);