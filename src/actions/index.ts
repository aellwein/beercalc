import { Action, ActionType, Gravity, HopsForm, MassUnit, Unit, VolumeMeasuredAt } from "../types";

export const changeUnit = (unit: Unit): Action => {
    return {
        type: ActionType.ChangeUnit,
        payload: unit
    };
}

export const changeUnit2 = (unit: Unit): Action => {
    return {
        type: ActionType.ChangeUnit2,
        payload: unit
    };
}

export const setOriginalGravity = (originalGravity: Gravity): Action => {
    return {
        type: ActionType.SetOriginalGravity,
        payload: originalGravity
    };
}

export const setOriginalGravity2 = (originalGravity: Gravity): Action => {
    return {
        type: ActionType.SetOriginalGravity2,
        payload: originalGravity
    };
}

export const setFinalGravity = (fg: Gravity): Action => {
    return {
        type: ActionType.FinalGravity,
        payload: fg
    };
}

export const boilingTime = (boilingTime: number): Action => {
    return {
        type: ActionType.BoilingTime,
        payload: boilingTime,
    };
}

export const changeVolume = (volume: number): Action => {
    return {
        type: ActionType.Volume,
        payload: volume
    };
}

export const changeFlameout = (flameout: number): Action => {
    return {
        type: ActionType.FlameOut,
        payload: flameout
    };
}

export const changeFlameoutTemp = (flameoutTemp: number): Action => {
    return {
        type: ActionType.FlameOutTemp,
        payload: flameoutTemp
    };
}

export const newHopsAddition = (): Action => {
    return {
        type: ActionType.NewHopsAddition,
        payload: null
    };
}

export const removeHopsAddition = (idx: number): Action => {
    return {
        type: ActionType.RemoveHopsAddition,
        payload: idx
    };
}

export const changeHopsAmount = (idx: number, amount: number): Action => {
    return {
        type: ActionType.ChangeHopsAmount,
        payload: { idx, amount }
    };
}

export const changeHopsAlpha = (idx: number, alpha: number): Action => {
    return {
        type: ActionType.ChangeHopsAlpha,
        payload: { idx, alpha }
    };
}

export const changeHopsBoil = (idx: number, boil: number): Action => {
    return {
        type: ActionType.ChangeHopsBoil,
        payload: { idx, boil }
    };
}

export const changeHopsForm = (idx: number, form: HopsForm): Action => {
    return {
        type: ActionType.ChangeHopsForm,
        payload: { idx, form }
    };
}

export const newMaltAddition = (): Action => {
    return {
        type: ActionType.NewMaltAddition,
        payload: null
    }
}

export const removeMaltAddition = (idx: number): Action => {
    return {
        type: ActionType.RemoveMaltAddition,
        payload: idx
    }
}

export const changeMaltMass = (idx: number, mass: number): Action => {
    return {
        type: ActionType.ChangeMaltMass,
        payload: { idx, mass }
    }
}

export const changeMaltMassUnit = (idx: number, massUnit: MassUnit): Action => {
    return {
        type: ActionType.ChangeMaltMassUnit,
        payload: { idx, massUnit }
    }
}

export const changeMaltColor = (idx: number, color: number): Action => {
    return {
        type: ActionType.ChangeMaltColor,
        payload: { idx, color }
    }
}

export const setGrainMassAndUnit = (newMass: number, newUnit: MassUnit): Action => {
    return {
        type: ActionType.SetGrainMassAndUnit,
        payload: { grainMass: newMass, grainMassUnit: newUnit }
    }
}

export const changeWortVolume = (wortVolume: number): Action => {
    return {
        type: ActionType.ChangeWortVolume,
        payload: { wortVolume }
    }
}

export const changeWortVolume2 = (wortVolume: number): Action => {
    return {
        type: ActionType.ChangeWortVolume2,
        payload: { wortVolume }
    }
}

export const changeVolumeMeasuredAt = (volumeMeasuredAt: VolumeMeasuredAt): Action => {
    return {
        type: ActionType.ChangeVolumeMeasuredAt,
        payload: { volumeMeasuredAt }
    }
}

export const changeVolumeMeasuredAt2 = (volumeMeasuredAt: VolumeMeasuredAt): Action => {
    return {
        type: ActionType.ChangeVolumeMeasuredAt2,
        payload: { volumeMeasuredAt }
    }
}

export const turnDarkMode = (): Action => {
    return {
        type: ActionType.TurnDarkMode,
        payload: null
    }
}

export const turnLiteMode = (): Action => {
    return {
        type: ActionType.TurnLiteMode,
        payload: null
    }
}

export const changeBrewhouseEfficiency = (be: number): Action => {
    return {
        type: ActionType.ChangeBrewhouseEfficiency,
        payload: { brewhouseEfficiency: be }
    }
}