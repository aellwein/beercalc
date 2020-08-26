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
        yield (
            <div className="columns is-multiline is-vcentered" key={"hops-" + i}>
                <div className="column has-text-right">{t('hops addition') + ' #' + (i + 1)}</div>
                <div className="column has-text-right">
                    <div className="select">
                        <select defaultValue="whole" onChange={(e) => onChangeForm(props, i, e.target.value)}>
                            <option value="whole">{t('whole')}</option>
                            <option value="plugs">{t('plugs')}</option>
                        </select>
                    </div>
                </div>
                <div className="column has-text-right">
                    <input className="input" type="number" min="0" max="5000" step=".1" value={a.amount} onChange={(e) => onChangeAmount(props, i, e.target.value)}></input>
                </div>
                <div className="column has-text-right">
                    <input className="input" type="number" min="0" max="90" step=".1" value={a.alpha} onChange={(e) => onChangeAlpha(props, i, e.target.value)}></input>
                </div>
                <div className="column has-text-right">
                    <input className="input" type="number" min="0" max="600" value={a.boil} onChange={(e) => onChangeBoil(props, i, e.target.value)}></input>
                </div>
                <div className="column has-text-right">
                    <div className="">{a.ibu}</div>
                </div>
                <div className="column is-1">
                    <div className="button" onClick={() => onRemoveHopsClick(props, i)}>X</div>
                </div>
            </div>
        );
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
        <div className="card">
            <div className="card-content">
                <div className="columns is-multiline">
                    <div className="column">&nbsp;</div>
                    <div className="column has-text-right">{t('form')}</div>
                    <div className="column has-text-right">{t('amount')}</div>
                    <div className="column has-text-right">{t('alpha acid')}</div>
                    <div className="column has-text-right">{t('boil time')}</div>
                    <div className="column has-text-right">{t('bitterness')}</div>
                    <div className="column is-1">&nbsp;</div>
                </div>
                {[...hopsAdditions(props, t)]}
                <div className="columns is-multiline is-vcentered">
                    <div className="column has-text-right"><div className="button is-primary" onClick={() => onAddHopsClick(props)}>{t('add hops')}</div></div>
                    <div className="column has-text-right"></div>
                    <div className="column has-text-right"></div>
                    <div className="column has-text-right"></div>
                    <div className="column has-text-right"></div>
                    <div className="column has-text-right"><strong>{calcBitterness(props)}</strong></div>
                    <div className="column is-1"><strong>IBU</strong></div>
                </div>
            </div>
        </div>
    );
}

const mapStateToProps = (state, ownProps) => {
    recalcIbu(state.beerCalc);
    return state.beerCalc;
}

export default connect(mapStateToProps, { newHopsAddition, removeHopsAddition, changeHopsAmount, changeHopsForm, changeHopsAlpha, changeHopsBoil })(HopsAdder);