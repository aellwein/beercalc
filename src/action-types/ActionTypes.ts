import { Gravity, HopsForm, MassUnit, Unit, VolumeMeasuredAt } from "../types";

export enum ActionType {
    ChangeUnit = "ChangeUnit",
    ChangeUnit2 = "ChangeUnit2",
    SetOriginalGravity = "SetOriginalGravity",
    SetOriginalGravity2 = "SetOriginalGravity2",
    SetFinalGravity = "SetFinalGravity",
    SetBoilingTime = "SetBoilingTime",
    SetVolume = "SetVolume",
    SetFlameOut = "SetFlameOut",
    SetFlameOutTemp = "SetFlameOutTemp",
    NewHopsAddition = "NewHopsAddition",
    RemoveHopsAddition = "RemoveHopsAddition",
    ChangeHopsAmount = "ChangeHopsAmount",
    ChangeHopsAlpha = "ChangeHopsAlpha",
    ChangeHopsBoil = "ChangeHopsBoil",
    ChangeHopsForm = "ChangeHopsForm",
    NewMaltAddition = "NewMaltAddition",
    RemoveMaltAddition = "RemoveMaltAddition",
    ChangeMaltMass = "ChangeMaltMass",
    ChangeMaltMassUnit = "ChangeMaltMassUnit",
    ChangeMaltColor = "ChangeMaltColor",
    SetGrainMassAndUnit = "SetGrainMassAndUnit",
    ChangeWortVolume = "ChangeWortVolume",
    ChangeWortVolume2 = "ChangeWortVolume2",
    ChangeVolumeMeasuredAt = "ChangeVolumeMeasuredAt",
    ChangeVolumeMeasuredAt2 = "ChangeVolumeMeasuredAt2",
    ChangeBrewhouseEfficiency = "ChangeBrewhouseEfficiency",
    TurnDarkMode = "TurnDarkMode",
    TurnLiteMode = "TurnLiteMode",
}

export interface ChangeUnitAction {
    type: ActionType.ChangeUnit;
    payload: Unit;
}
export interface ChangeUnit2Action {
    type: ActionType.ChangeUnit2;
    payload: Unit;
}
export interface SetOriginalGravityAction {
    type: ActionType.SetOriginalGravity;
    payload: Gravity;
}
export interface SetOriginalGravity2Action {
    type: ActionType.SetOriginalGravity2;
    payload: Gravity;
}
export interface SetFinalGravityAction {
    type: ActionType.SetFinalGravity;
    payload: Gravity;
}
export interface SetBoilingTimeAction {
    type: ActionType.SetBoilingTime;
    payload: number;
}
export interface SetVolumeAction {
    type: ActionType.SetVolume;
    payload: number;
}
export interface SetFlameOutAction {
    type: ActionType.SetFlameOut;
    payload: number;
}
export interface SetFlameOutTempAction {
    type: ActionType.SetFlameOutTemp;
    payload: number;
}
export interface NewHopsAdditionAction {
    type: ActionType.NewHopsAddition;
}
export interface RemoveHopsAdditionAction {
    type: ActionType.RemoveHopsAddition;
    payload: number;
}
export interface HopsAmount {
    idx: number;
    amount: number;
}
export interface ChangeHopsAmountAction {
    type: ActionType.ChangeHopsAmount;
    payload: HopsAmount;
}
export interface HopsAlpha {
    idx: number;
    alpha: number;
}
export interface ChangeHopsAlphaAction {
    type: ActionType.ChangeHopsAlpha;
    payload: HopsAlpha;
}
export interface HopsBoil {
    idx: number;
    boil: number;
}
export interface ChangeHopsBoilAction {
    type: ActionType.ChangeHopsBoil;
    payload: HopsBoil;
}
export interface ChangeHopsFormActionPayload {
    idx: number;
    form: HopsForm;
}
export interface ChangeHopsFormAction {
    type: ActionType.ChangeHopsForm;
    payload: ChangeHopsFormActionPayload;
}
export interface NewMaltAdditionAction {
    type: ActionType.NewMaltAddition;
}
export interface RemoveMaltAdditionAction {
    type: ActionType.RemoveMaltAddition;
    payload: number;
}
export interface ChangeMaltMassActionPayload {
    idx: number;
    mass: number;
}
export interface ChangeMaltMassAction {
    type: ActionType.ChangeMaltMass;
    payload: ChangeMaltMassActionPayload;
}
export interface ChangeMaltMassUnitActionPayload {
    idx: number;
    massUnit: MassUnit
}
export interface ChangeMaltMassUnitAction {
    type: ActionType.ChangeMaltMassUnit;
    payload: ChangeMaltMassUnitActionPayload;
}
export interface ChangeMaltColorActionPayload {
    idx: number;
    color: number;
}
export interface ChangeMaltColorAction {
    type: ActionType.ChangeMaltColor;
    payload: ChangeMaltColorActionPayload;
}
export interface GrainMassAndUnit {
    grainMass: number;
    grainMassUnit: MassUnit;
}
export interface SetGrainMassAndUnitAction {
    type: ActionType.SetGrainMassAndUnit;
    payload: GrainMassAndUnit;
}
export interface ChangeWortVolumeAction {
    type: ActionType.ChangeWortVolume;
    payload: number;
}
export interface ChangeWortVolume2Action {
    type: ActionType.ChangeWortVolume2;
    payload: number;
}
export interface ChangeVolumeMeasuredAtAction {
    type: ActionType.ChangeVolumeMeasuredAt;
    payload: VolumeMeasuredAt;
}
export interface ChangeVolumeMeasuredAt2Action {
    type: ActionType.ChangeVolumeMeasuredAt2;
    payload: VolumeMeasuredAt;
}
export interface ChangeBrewhouseEfficiencyAction {
    type: ActionType.ChangeBrewhouseEfficiency;
    payload: number;
}
export interface TurnDarkModeAction {
    type: ActionType.TurnDarkMode;
}
export interface TurnLiteModeAction {
    type: ActionType.TurnLiteMode;
}


export type Action =
    ChangeUnitAction |
    ChangeUnit2Action |
    SetOriginalGravityAction |
    SetOriginalGravity2Action |
    SetFinalGravityAction |
    SetBoilingTimeAction |
    SetVolumeAction |
    SetFlameOutAction |
    SetFlameOutTempAction |
    NewHopsAdditionAction |
    RemoveHopsAdditionAction |
    ChangeHopsAmountAction |
    ChangeHopsAlphaAction |
    ChangeHopsBoilAction |
    ChangeHopsFormAction |
    NewMaltAdditionAction |
    RemoveMaltAdditionAction |
    ChangeMaltMassAction |
    ChangeMaltMassUnitAction |
    ChangeMaltColorAction |
    SetGrainMassAndUnitAction |
    ChangeWortVolumeAction |
    ChangeWortVolume2Action |
    ChangeVolumeMeasuredAtAction |
    ChangeVolumeMeasuredAt2Action |
    ChangeBrewhouseEfficiencyAction |
    TurnDarkModeAction |
    TurnLiteModeAction;
