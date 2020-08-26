import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import './AlcoholCalc.css';
import CalcStandard from './CalcStandard';
import CalcTerrill from './CalcTerrill';
import GravityPicker from './GravityPicker';
import UnitPicker from './UnitPicker';

const AlcoholCalc = (_) => {
    const { t } = useTranslation();

    return (
        <div className="container top-title">
            <div className="columns">
                <div className="column">
                    <div className="title is-4">{t('alcohol calculator')}</div>
                </div>
            </div>
            <div className="columns">
                <div className="column is-full"><UnitPicker /></div>
            </div>
            <div className="columns">
                <div className="column is-full"><GravityPicker /></div>
            </div>
            <div className="columns">
                <div className="column is-half"><CalcStandard /></div>
                <div className="column is-half"><CalcTerrill /></div>
            </div>
        </div>
    );
}

const mapStateToProps = (state) => {
    return state.beerCalc;
}

export default connect(mapStateToProps, {})(AlcoholCalc);