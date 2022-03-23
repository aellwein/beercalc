import { connect } from 'react-redux';
import { CalculatorState, ColorCalcState, Gravity, MassUnit, Unit } from '../../types';
import { convertUnits } from '../units/calculations';
import './EBC.css';

interface ColorDisplayProps {
    unit: Unit;
    originalGravity: Gravity;
    boiling: number;
    grain: ColorCalcState;
}

const calculateEbc = (props: ColorDisplayProps): number => {
    const og_plato = convertUnits(props.originalGravity.amount, props.originalGravity.unit).plato;
    let totalMass = 0;
    let prodMassEbc = 0;
    for (let i = 0; i < props.grain.malt.length; i++) {
        const factor = (props.grain.malt[i].massUnit === MassUnit.Kilogram) ? 1 : 0.001;
        totalMass += props.grain.malt[i].amount * factor;
        prodMassEbc += props.grain.malt[i].amount * factor * props.grain.malt[i].color;
    }
    const ebc = (prodMassEbc / totalMass) * og_plato / 10 + 1.5 * props.boiling / 60;
    return Number.parseFloat(ebc.toFixed(0));
}

const ColorDisplay = (props: ColorDisplayProps) => {
    if (props.grain.malt.length === 0) {
        return <div></div>;
    }

    const ebc = calculateEbc(props);
    const ebcClassName = (ebc >= 100) ? "ebc-100" : "ebc-" + ebc;
    return (
        <div className="flex flex-row shadow-md p-4 gap-4 items-baseline">
            <div className='my-auto grow'><strong>{ebc} EBC</strong></div>
            <div className={ebcClassName + " grow"} style={{ height: "100px" }}></div>
        </div>
    );
}

const mapStateToProps = (rootState: any) => {
    const state: CalculatorState = rootState.beerCalc;
    return {
        boiling: state.ibu.boiling,
        grain: state.grain,
        originalGravity: state.originalGravity,
        unit: state.unit
    };
}

export default connect(mapStateToProps, {})(ColorDisplay);