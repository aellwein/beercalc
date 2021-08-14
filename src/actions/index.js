import { BOILING_TIME, CHANGE_BREWHOUSE_EFFICIENCY, CHANGE_HOPS_ALPHA, CHANGE_HOPS_AMOUNT, CHANGE_HOPS_BOIL, CHANGE_HOPS_FORM, CHANGE_MALT_COLOR, CHANGE_MALT_MASS, CHANGE_MALT_MASS_UNIT, CHANGE_UNIT, CHANGE_UNIT_2, CHANGE_VOLUME_MEASURED_AT, CHANGE_VOLUME_MEASURED_AT_2, CHANGE_WORT_VOLUME, CHANGE_WORT_VOLUME_2, FINAL_GRAVITY, FLAMEOUT, FLAMEOUT_TEMP, NEW_HOPS_ADDITION, NEW_MALT_ADDITION, ORIGINAL_GRAVITY, ORIGINAL_GRAVITY_2, REMOVE_HOPS_ADDITION, REMOVE_MALT_ADDITION, SET_GRAIN_MASS_AND_UNIT, TURN_DARK_MODE, TURN_LITE_MODE, VOLUME } from "./types";

export const changeUnit = (unit) => {
    return {
        type: CHANGE_UNIT,
        payload: unit
    };
}

export const changeUnit2 = (unit) => {
    return {
        type: CHANGE_UNIT_2,
        payload: unit
    };
}

export const originalGravity = (newValue) => {
    return {
        type: ORIGINAL_GRAVITY,
        payload: newValue
    };
}

export const originalGravity2 = (newValue) => {
    return {
        type: ORIGINAL_GRAVITY_2,
        payload: newValue
    };
}

export const finalGravity = (newValue) => {
    return {
        type: FINAL_GRAVITY,
        payload: newValue
    };
}

export const boilingTime = (newValue) => {
    return {
        type: BOILING_TIME,
        payload: newValue
    };
}

export const changeVolume = (newValue) => {
    return {
        type: VOLUME,
        payload: newValue
    };
}

export const changeFlameout = (newValue) => {
    return {
        type: FLAMEOUT,
        payload: newValue
    };
}

export const changeFlameoutTemp = (newValue) => {
    return {
        type: FLAMEOUT_TEMP,
        payload: newValue
    };
}

export const newHopsAddition = () => {
    return {
        type: NEW_HOPS_ADDITION,
        payload: null
    };
}

export const removeHopsAddition = (idx) => {
    return {
        type: REMOVE_HOPS_ADDITION,
        payload: idx
    };
}

export const changeHopsAmount = (idx, amount) => {
    return {
        type: CHANGE_HOPS_AMOUNT,
        payload: { idx, amount }
    };
}

export const changeHopsAlpha = (idx, alpha) => {
    return {
        type: CHANGE_HOPS_ALPHA,
        payload: { idx, alpha }
    };
}

export const changeHopsBoil = (idx, boil) => {
    return {
        type: CHANGE_HOPS_BOIL,
        payload: { idx, boil }
    };
}

export const changeHopsForm = (idx, form) => {
    return {
        type: CHANGE_HOPS_FORM,
        payload: { idx, form }
    };
}

export const newMaltAddition = () => {
    return {
        type: NEW_MALT_ADDITION,
        payload: null
    }
}

export const removeMaltAddition = (idx) => {
    return {
        type: REMOVE_MALT_ADDITION,
        payload: idx
    }
}

export const changeMaltMass = (idx, mass) => {
    return {
        type: CHANGE_MALT_MASS,
        payload: { idx, mass }
    }
}

export const changeMaltMassUnit = (idx, massUnit) => {
    return {
        type: CHANGE_MALT_MASS_UNIT,
        payload: { idx, massUnit }
    }
}

export const changeMaltColor = (idx, color) => {
    return {
        type: CHANGE_MALT_COLOR,
        payload: { idx, color }
    }
}

export const setGrainMassAndUnit = (newMass, newUnit) => {
    return {
        type: SET_GRAIN_MASS_AND_UNIT,
        payload: { grainMass: newMass, grainMassUnit: newUnit }
    }
}

export const changeWortVolume = (wortVolume) => {
    return {
        type: CHANGE_WORT_VOLUME,
        payload: { wortVolume }
    }
}

export const changeWortVolume2 = (wortVolume) => {
    return {
        type: CHANGE_WORT_VOLUME_2,
        payload: { wortVolume }
    }
}

export const changeVolumeMeasuredAt = (volumeMeasuredAt) => {
    return {
        type: CHANGE_VOLUME_MEASURED_AT,
        payload: { volumeMeasuredAt }
    }
}

export const changeVolumeMeasuredAt2 = (volumeMeasuredAt) => {
    return {
        type: CHANGE_VOLUME_MEASURED_AT_2,
        payload: { volumeMeasuredAt }
    }
}

export const turnDarkMode = () => {
    return {
        type: TURN_DARK_MODE,
        payload: null
    }
}

export const turnLiteMode = () => {
    return {
        type: TURN_LITE_MODE,
        payload: null
    }
}

export const changeBrewhouseEfficiency = (be) => {
    return {
        type: CHANGE_BREWHOUSE_EFFICIENCY,
        payload: { brewhouseEfficiency: be }
    }
}