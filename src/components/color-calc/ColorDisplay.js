import React from 'react';
import { connect } from 'react-redux';
import { convertUnits } from '../units/calculations';
import './EBC.css';

const calculateEbc = (props) => {
    const og_plato = convertUnits(props.gravity.original, props.unit).plato;
    let totalMass = 0;
    let prodMassEbc = 0;
    for (let i = 0; i < props.grain.malt.length; i++) {
        const factor = (props.grain.malt[i].massUnit === 'kg') ? 1 : 0.001;
        totalMass += props.grain.malt[i].mass * factor;
        prodMassEbc += props.grain.malt[i].mass * factor * props.grain.malt[i].color;
    }
    const ebc = (prodMassEbc / totalMass) * og_plato / 10 + 1.5 * props.boiling / 60;
    return ebc.toFixed(0);
}

const ColorDisplay = (props) => {
    if (typeof (props.grain.malt) === 'undefined' || props.grain.malt.length === 0) {
        return <div></div>;
    }

    const ebc = calculateEbc(props);

    return (
        <div className="columns is-vcentered">
            <div className="column is-half">
                <strong>{ebc} EBC</strong>
            </div>
            <div className="column is-half">
                <div className={(ebc >= 100) ? "ebc-100" : "ebc-" + ebc} style={{ height: "100px" }}></div>
            </div>
        </div>
    );
}

const mapStateToProps = (state) => {
    return {
        boiling: state.beerCalc.ibu.boiling,
        grain: state.beerCalc.grain,
        gravity: state.beerCalc.gravity,
        unit: state.beerCalc.unit
    };
}

export default connect(mapStateToProps, {})(ColorDisplay);