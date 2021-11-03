export enum ActionType {
    ChangeUnit = "ChangeUnit",
    ChangeUnit2 = "ChangeUnit2",
    SetOriginalGravity = "SetOriginalGravity",
    SetOriginalGravity2 = "SetOriginalGravity2",
    FinalGravity = "FinalGravity",
    BoilingTime = "BoilingTime",
    Volume = "Volume",
    FlameOut = "FlameOut",
    FlameOutTemp = "FlameOutTemp",
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

export interface Action {
    payload: any;
    type: ActionType;
}

export enum DisplayMode {
    Lite = 'lite',
    Dark = 'dark',
}

export enum Unit {
    Brix = 'brix',
    SG = 'sg',
    Plato = 'plato',
    Oechsle = 'oe',
}

export interface Gravity {
    amount: number;
    unit: Unit;
}

export enum HopsForm {
    Whole = 'whole',
    Plugs = 'plugs',
}

export interface HopsAddition {
    form: HopsForm;
    amount: number;
    alpha: number;
    boil: number;
    ibu: number;
}

export interface IbuState {
    boiling: number;
    volume: number;
    flameout: number;
    flameoutTemp: number;
    hops: HopsAddition[];
}

export enum MassUnit {
    Kilogram = 'kg',
    Gram = 'g',
}

export interface MaltAddition {
    amount: number;
    massUnit: MassUnit;
    color: number;
}

export enum VolumeMeasuredAt {
    Hundred = '100',
    Twenty = '20',
}

export interface GrainMassFromBrewhouse {
    originalGravity: Gravity;
    wortVolume: number;
    volumeMeasuredAt: VolumeMeasuredAt;
    brewhouseEfficiency: number;
}

export interface BrewhouseEfficiencyState {
    volumeMeasuredAt: VolumeMeasuredAt;
    grainMassFromBrewhouse: GrainMassFromBrewhouse;
    grainMassUnit: MassUnit;
    grainMass: number;
    wortVolume: number;
}

export interface ColorCalcState {
    malt: MaltAddition[];
}

export interface CalculatorState {
    displayMode: DisplayMode;
    unit: Unit;
    originalGravity: Gravity;
    finalGravity: Gravity;
    ibu: IbuState;
    grain: ColorCalcState;
    brewhouse: BrewhouseEfficiencyState;
}

