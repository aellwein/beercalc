import { CHANGE_UNIT, ORIGINAL_GRAVITY, FINAL_GRAVITY, BOILING_TIME, VOLUME, FLAMEOUT, FLAMEOUT_TEMP, NEW_HOPS_ADDITION, REMOVE_HOPS_ADDITION, CHANGE_HOPS_AMOUNT, CHANGE_HOPS_ALPHA, CHANGE_HOPS_BOIL, CHANGE_HOPS_FORM } from "./types"

export const changeUnit = (unit) => {
    return {
        type: CHANGE_UNIT,
        payload: unit
    };
}

export const originalGravity = (newValue) => {
    return {
        type: ORIGINAL_GRAVITY,
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
