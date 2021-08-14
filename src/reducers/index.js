import { combineReducers } from "redux";
import { BOILING_TIME, CHANGE_BREWHOUSE_EFFICIENCY, CHANGE_HOPS_ALPHA, CHANGE_HOPS_AMOUNT, CHANGE_HOPS_BOIL, CHANGE_HOPS_FORM, CHANGE_MALT_COLOR, CHANGE_MALT_MASS, CHANGE_MALT_MASS_UNIT, CHANGE_UNIT, CHANGE_UNIT_2, CHANGE_VOLUME_MEASURED_AT, CHANGE_VOLUME_MEASURED_AT_2, CHANGE_WORT_VOLUME, CHANGE_WORT_VOLUME_2, FINAL_GRAVITY, FLAMEOUT, FLAMEOUT_TEMP, NEW_HOPS_ADDITION, NEW_MALT_ADDITION, ORIGINAL_GRAVITY, ORIGINAL_GRAVITY_2, REMOVE_HOPS_ADDITION, REMOVE_MALT_ADDITION, SET_GRAIN_MASS_AND_UNIT, TURN_DARK_MODE, TURN_LITE_MODE, VOLUME } from "../actions/types";

const defaultState = {
    displayMode: 'lite',
    unit: 'brix',
    gravity: {
        original: 12.0,
        final: 6.0,
    },
    ibu: {
        boiling: 90,
        volume: 50,
        flameout: 5,
        flameoutTemp: 85,
        hops: [],
    },
    grain: {
        malt: [],
    },
    brewhouse: {
        volumeMeasuredAt: '100',
        grainMassFromBrewhouse: {
            originalGravity: 12,
            originalGravityUnit: 'brix',
            wortVolume: 20,
            volumeMeasuredAt: '100',
            brewhouseEfficiency: 65
        }
    },
};

const beerCalc = (state = defaultState, action) => {
    switch (action.type) {
        case CHANGE_UNIT:
            return { ...state, unit: action.payload };
        case CHANGE_UNIT_2:
            return { ...state, brewhouse: { ...state.brewhouse, grainMassFromBrewhouse: { ...state.brewhouse.grainMassFromBrewhouse, originalGravityUnit: action.payload } } };
        case ORIGINAL_GRAVITY:
            return { ...state, gravity: { ...state.gravity, original: action.payload } };
        case ORIGINAL_GRAVITY_2:
            return { ...state, brewhouse: { ...state.brewhouse, grainMassFromBrewhouse: { ...state.brewhouse.grainMassFromBrewhouse, originalGravity: action.payload } } };
        case FINAL_GRAVITY:
            return { ...state, gravity: { ...state.gravity, final: action.payload } };
        case BOILING_TIME:
            // we have to check the hops additions here, to reduce the
            // boiling time of the particular hops addition to be not longer then the total boiling time.
            if (state.ibu.hops) {
                for (let h of state.ibu.hops) {
                    h.boil = Math.min(h.boil, action.payload);
                }
            }
            return { ...state, ibu: { ...state.ibu, boiling: action.payload } };
        case VOLUME:
            return { ...state, ibu: { ...state.ibu, volume: action.payload } };
        case FLAMEOUT:
            return { ...state, ibu: { ...state.ibu, flameout: action.payload } };
        case FLAMEOUT_TEMP:
            return { ...state, ibu: { ...state.ibu, flameoutTemp: action.payload } };
        case NEW_HOPS_ADDITION:
            let ha = {
                form: 'whole',
                amount: 5.0,
                alpha: 7.0,
                boil: 90,
                ibu: 0
            };
            return { ...state, ibu: { ...state.ibu, hops: [...state.ibu.hops, ha] } };
        case REMOVE_HOPS_ADDITION:
            state.ibu.hops.splice(action.payload, 1);
            return { ...state, ibu: { ...state.ibu, hops: [...state.ibu.hops] } };
        case CHANGE_HOPS_AMOUNT:
            let newHops = state.ibu.hops;
            newHops[action.payload.idx].amount = action.payload.amount;
            return { ...state, ibu: { ...state.ibu, hops: newHops } };
        case CHANGE_HOPS_ALPHA:
            let hops = state.ibu.hops;
            hops[action.payload.idx].alpha = action.payload.alpha;
            return { ...state, ibu: { ...state.ibu, hops } };
        case CHANGE_HOPS_BOIL:
            let h = state.ibu.hops;
            h[action.payload.idx].boil = action.payload.boil;
            return { ...state, ibu: { ...state.ibu, hops: h } };
        case CHANGE_HOPS_FORM:
            let ho = state.ibu.hops;
            ho[action.payload.idx].form = action.payload.form;
            return { ...state, ibu: { ...state.ibu, hops: ho } };
        case NEW_MALT_ADDITION:
            let malt = { mass: 5.0, massUnit: 'kg', color: 8 };
            return { ...state, grain: { ...state.grain, malt: [...state.grain.malt, malt] } };
        case REMOVE_MALT_ADDITION:
            state.grain.malt.splice(action.payload, 1);
            return { ...state, grain: { ...state.grain, malt: [...state.grain.malt] } };
        case CHANGE_MALT_MASS:
            let newMalt = state.grain.malt;
            newMalt[action.payload.idx].mass = action.payload.mass;
            return { ...state, grain: { ...state.grain, malt: newMalt } };
        case CHANGE_MALT_MASS_UNIT:
            let newMalt2 = state.grain.malt;
            newMalt2[action.payload.idx].massUnit = action.payload.massUnit;
            return { ...state, grain: { ...state.grain, malt: newMalt2 } };
        case CHANGE_MALT_COLOR:
            let newMalt3 = state.grain.malt;
            newMalt3[action.payload.idx].color = action.payload.color;
            return { ...state, grain: { ...state.grain, malt: newMalt3 } };
        case TURN_DARK_MODE:
            return { ...state, displayMode: 'dark' };
        case TURN_LITE_MODE:
            return { ...state, displayMode: 'lite' };
        case SET_GRAIN_MASS_AND_UNIT:
            return { ...state, brewhouse: { ...state.brewhouse, grainMass: action.payload.grainMass, grainMassUnit: action.payload.grainMassUnit } };
        case CHANGE_WORT_VOLUME:
            return { ...state, brewhouse: { ...state.brewhouse, wortVolume: action.payload.wortVolume } };
        case CHANGE_WORT_VOLUME_2:
            return { ...state, brewhouse: { ...state.brewhouse, grainMassFromBrewhouse: { ...state.brewhouse.grainMassFromBrewhouse, wortVolume: action.payload.wortVolume } } };
        case CHANGE_VOLUME_MEASURED_AT:
            return { ...state, brewhouse: { ...state.brewhouse, volumeMeasuredAt: action.payload.volumeMeasuredAt } };
        case CHANGE_VOLUME_MEASURED_AT_2:
            return { ...state, brewhouse: { ...state.brewhouse, grainMassFromBrewhouse: { ...state.brewhouse.grainMassFromBrewhouse, volumeMeasuredAt: action.payload.volumeMeasuredAt } } };
        case CHANGE_BREWHOUSE_EFFICIENCY:
            return { ...state, brewhouse: { ...state.brewhouse, grainMassFromBrewhouse: { ...state.brewhouse.grainMassFromBrewhouse, brewhouseEfficiency: action.payload.brewhouseEfficiency } } };
        default:
            return state;
    }
}

export default combineReducers({
    beerCalc
});