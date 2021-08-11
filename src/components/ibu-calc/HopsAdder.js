import React from "react";
import { useTranslation } from "react-i18next";
import { connect } from "react-redux";
import { changeHopsAlpha, changeHopsAmount, changeHopsForm, changeHopsBoil, newHopsAddition, removeHopsAddition } from '../../actions';
import { recalcIbu } from "./IbuCalc";

const onAddHopsClick = (props) => {
    props.newHopsAddition();
}

const onRemoveHopsClick = (props, idx) => {
    props.removeHopsAddition(idx);
}

const onChangeAmount = (props, idx, amount) => {
    let a = parseFloat(amount);
    if (isNaN(a)) {
        return;
    }
    if (a < 0) {
        a = 0;
    }
    if (a > 5000) {
        a = 5000;
    }
    if (a !== null) {
        props.changeHopsAmount(idx, a);
    }
}

const onChangeAlpha = (props, idx, newAlpha) => {
    let a = parseFloat(newAlpha);
    if (isNaN(a)) {
        return;
    }
    if (a < 0) {
        a = 0;
    }
    if (a > 90) {
        a = 90;
    }
    if (a !== null) {
        props.changeHopsAlpha(idx, a);
    }
}

const onChangeBoil = (props, idx, boilTime) => {
    let b = parseFloat(boilTime);
    if (isNaN(b)) {
        return;
    }
    if (b < 0) {
        b = 0;
    }
    if (b > Math.min(b, props.ibu.boiling)) {
        b = Math.min(b, props.ibu.boiling)
    }
    if (b !== null) {
        props.changeHopsBoil(idx, b);
    }
}

const onChangeForm = (props, idx, form) => {
    props.changeHopsForm(idx, form);
}

const hopsAdditions = function* (props, t) {
    for (let i = 0; i < props.ibu.hops.length; i++) {
        let a = props.ibu.hops[i];
        yield <div className="underline 2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right" key={i + "-col1-1"}>{t('hops addition') + ' #' + (i + 1)}</div>;
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left" key={i + "col1-2"}>
                <div className="">{a.ibu} <button className="py-2 px-2 bg-red-300 text-white hover:bg-red-600 p-1" onClick={() => onRemoveHopsClick(props, i)}>X</button></div>
            </div>
        );
        yield <div key={i + "-col2-1"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('form')}</div>;
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left" key={i + "col2-2"}>
                <select className="p-1 border-gray-300 border-1 border-solid" defaultValue="whole" onChange={(e) => onChangeForm(props, i, e.target.value)}>
                    <option value="whole">{t('whole')}</option>
                    <option value="plugs">{t('plugs')}</option>
                </select>
            </div>
        );
        yield <div key={i + "-col3-1"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('amount')}</div>;
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left" key={i + "col3-2"}>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none" type="number" min="0" max="5000" step=".1" value={a.amount} onChange={(e) => onChangeAmount(props, i, e.target.value)}></input>
            </div>
        );
        yield <div key={i + "-col4-1"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('alpha acid')}</div>;
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left" key={i + "col4-2"}>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none" type="number" min="0" max="90" step=".1" value={a.alpha} onChange={(e) => onChangeAlpha(props, i, e.target.value)}></input>
            </div>
        );
        yield <div key={i + "-col5-1"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('boil time')}</div>;
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left" key={i + "col5-2"}>
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none" type="number" min="0" max="600" value={a.boil} onChange={(e) => onChangeBoil(props, i, e.target.value)}></input>
            </div>
        );
        yield <div key={i + "-col6-1"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('bitterness')}</div>;
        yield <div key={i + "-col6-2"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{a.ibu} IBU</div>
    }
}

const calcBitterness = (props) => {
    let b = 0.0;
    if (props.ibu.hops.length === 0) {
        return b;
    }
    for (let h of props.ibu.hops) {
        b += parseFloat(h.ibu);
    }
    return b.toFixed(1);
}

const HopsAdder = (props) => {
    const { t } = useTranslation();
    return (
        <div className="grid grid-cols-12 gap-4 p-4 shadow-md items-baseline">
            {[...hopsAdditions(props, t)]}
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-6 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right"><strong>{t('total bitterness')}</strong></div>
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-6 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right"><strong>{calcBitterness(props)} IBU</strong></div>
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-6 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">
                <button className="py-2 px-4 text-white bg-green-500 rounded-lg shadow-md hover:bg-green-700" onClick={() => onAddHopsClick(props)}>{t('add hops')}</button>
            </div>
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-6 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right"></div>
        </div>
    );
}


const mapStateToProps = (state, _) => {
    recalcIbu(state.beerCalc);
    return state.beerCalc;
}

export default connect(mapStateToProps, { newHopsAddition, removeHopsAddition, changeHopsAmount, changeHopsForm, changeHopsAlpha, changeHopsBoil })(HopsAdder);