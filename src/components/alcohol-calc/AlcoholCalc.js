import React, { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import './AlcoholCalc.css';
import CalcStandard from './CalcStandard';
import CalcTerrill from './CalcTerrill';
import GravityPicker from './GravityPicker';
import UnitPicker from './UnitPicker';

const languages = function* () {
    const langs = [
        { 'short': 'en', 'long': 'English' },
        { 'short': 'de', 'long': 'Deutsch' },
        { 'short': 'ru', 'long': 'Русский' },
    ];
    for (let i of langs) {
        yield <option key={i.short} value={i.short}>{i.long}</option>
    }
}

const onChangeLang = (lang, i18n) => {
    i18n.changeLanguage(lang);
}

const AlcoholCalc = (_) => {
    const { t, i18n } = useTranslation();
    useEffect(() => {
        document.title = t('title');
    });
    return (
        <div className="container top-title">
            <div className="columns">
                <div className="column">
                    <div className="title is-4">{t('alcohol calculator')}</div>
                </div>
                <div className="column is-narrow">
                    <div className="select">
                        <select onChange={e => onChangeLang(e.target.value, i18n)} value={i18n.language}>
                            {[...languages()]}
                        </select>
                    </div>
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