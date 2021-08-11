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
        yield <div key={i + "col1-1"} className="underline 2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('malt') + ' #' + (i + 1)}</div>;
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left" key={i + "col1-2"}>
                <div className="">{a.ibu} <button className="py-2 px-2 bg-red-300 text-white hover:bg-red-600 p-1" onClick={() => onRemoveGrainClick(props, i)}>X</button></div>
            </div>
        );

        yield <div key={i + "-col2-1"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('mass')}</div>;
        yield (
            <div key={i + "-col2-2"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left">
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none" type="number" min="0" max="500" step=".1" value={a.mass} onChange={(e) => onChangeMass(props, i, e.target.value)}></input>
            </div>
        );
        yield <div key={i + "-col3-1"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('mass unit')}</div>;
        yield (
            <div key={i + "-col3-2"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left">
                <select className="p-1 border-gray-300 border-1 border-solid" defaultValue="kg" onChange={(e) => onChangeMassUnit(props, i, e.target.value)}>
                    <option value="kg">{t('kg')}</option>
                    <option value="g">{t('g')}</option>
                </select>
            </div>
        );
        yield <div key={i + "-col4-1"} className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-4 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">{t('color ebc')}</div>;
        yield (
            <div key={i + "-col-4-2"}
                className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-8 col-span-6 2xl:text-left xl:text-left lg:text-left md:text-left sm:text-left xs:text-left text-left">
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none" type="number" min="1" max="10000" step="1" value={a.color} onChange={(e) => onChangeEBC(props, i, e.target.value)}></input>
            </div>
        );
    }
}

const MaltAdder = (props) => {
    const { t } = useTranslation();
    return (
        <div className="grid grid-cols-12 gap-4 shadow-md p-4 items-baseline">
            {[...grainAdditions(props, t)]}
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-6 xs:col-span-6 col-span-6 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-right xs:text-right text-right">
                <button className="py-2 px-4 bg-yellow-500 text-white rounded-lg shadow-md hover:bg-yellow-700" onClick={() => onAddMalt(props)}>{t('add malt')}</button>
            </div>
        </div>
    );
}

const mapStateToProps = (state) => {
    return { grain: state.beerCalc.grain };
}

export default connect(mapStateToProps, { newMaltAddition, removeMaltAddition, changeMaltMass, changeMaltMassUnit, changeMaltColor })(MaltAdder);