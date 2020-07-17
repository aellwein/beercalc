import { CHANGE_UNIT, ORIGINAL_GRAVITY, FINAL_GRAVITY } from "./types"

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