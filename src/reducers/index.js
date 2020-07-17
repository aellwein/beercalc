import { combineReducers } from "redux";
import { CHANGE_UNIT, ORIGINAL_GRAVITY, FINAL_GRAVITY } from "../actions/types";

const defaultState = {
    unit: 'brix',
    gravity: {
        original: 12.0,
        final: 6.0
    }
};

const beerCalc = (state = defaultState, action) => {
    switch (action.type) {
        case CHANGE_UNIT:
            return { ...state, unit: action.payload };
        case ORIGINAL_GRAVITY:
            return { ...state, gravity: { ...state.gravity, original: action.payload } };
        case FINAL_GRAVITY:
            return { ...state, gravity: { ...state.gravity, final: action.payload } };
        default:
            return state;
    }
}

export default combineReducers({
    beerCalc
});