import { Gravity, Unit } from "../../types";

const brixToPlato = (brix: number): number => {
    return brix / 1.03;
}

const platoToBrix = (plato: number): number => {
    return plato * 1.03;
}

const platoToSg = (plato: number): number => {
    return 1 + (plato / (258.6 - ((plato / 258.2) * 227.1)));
}

const sgToPlato = (sg: number): number => {
    return 668.72 * sg - 463.37 - 205.347 * Math.pow(sg, 2);
}

const brixToOe = (brix: number): number => {
    return brix * 4.25;
}

const oeToBrix = (oe: number): number => {
    return oe / 4.25;
}

interface UnitConversion {
    brix: number;
    plato: number;
    sg: number;
    oe: number;
}

export const convertUnits = (amount: number, unit: Unit): UnitConversion => {
    switch (unit) {
        case Unit.Brix:
            return { brix: amount, plato: brixToPlato(amount), sg: platoToSg(brixToPlato(amount)), oe: brixToOe(amount) };
        case Unit.Plato:
            return { brix: platoToBrix(amount), plato: amount, sg: platoToSg(amount), oe: brixToOe(platoToBrix(amount)) };
        case Unit.SG:
            return { brix: platoToBrix(sgToPlato(amount)), plato: sgToPlato(amount), sg: amount, oe: brixToOe(platoToBrix(sgToPlato(amount))) };
        case Unit.Oechsle:
            return { brix: oeToBrix(amount), plato: brixToPlato(oeToBrix(amount)), sg: platoToSg(brixToPlato(oeToBrix(amount))), oe: amount };
    }
}

const calcApparentExtractSG = (ogUnit: UnitConversion, fgUnit: UnitConversion): number => {
    return 1.001843 - 0.002318474 * ogUnit.plato - 0.000007775 * Math.pow(ogUnit.plato, 2) -
        0.000000034 * Math.pow(ogUnit.plato, 3) + 0.00574 * fgUnit.brix +
        0.00003344 * Math.pow(fgUnit.brix, 2) +
        0.000000086 * Math.pow(fgUnit.brix, 3);
}

const calcApparentExtractSGTerrill = (ogUnit: UnitConversion, fgUnit: UnitConversion): number => {
    return 1.0000 - 0.00085683 * ogUnit.plato + 0.0034941 * fgUnit.plato;
}

const calcRealExtract = (ogUnit: UnitConversion, apparentExPlato: number): number => {
    return 0.1808 * ogUnit.plato + 0.8192 * apparentExPlato;
}

const calcApparentAttenuation = (ogUnit: UnitConversion, apparentExPlato: number): number => {
    return (1 - apparentExPlato / ogUnit.plato) * 100;
}

const calcRealAttenuation = (ogUnit: UnitConversion, realExPlato: number): number => {
    return (1 - realExPlato / ogUnit.plato) * 100;
}

const calcAlcoholByWeight = (ogUnit: UnitConversion, realExPlato: number): number => {
    return (ogUnit.plato - realExPlato) / (2.0665 - 0.010665 * ogUnit.plato);
}

const calcAlcoholByVolume = (ogUnit: UnitConversion, realExPlato: number): number => {
    return 1 / 0.79 * (ogUnit.plato - realExPlato) / (2.0665 - 0.010665 * ogUnit.plato);
}

export interface AlcoholCalculationResult {
    apparentExtract: number;
    realExtract: number;
    apparentAttenuation: number;
    realAttenuation: number;
    alcByWeight: number;
    alcByVolume: number;
}

export const alcoholStandardFormula = (og: Gravity, fg: Gravity): AlcoholCalculationResult => {
    const ogUnit = convertUnits(og.amount, og.unit);
    const fgUnit = convertUnits(fg.amount, fg.unit);
    const apparentExtract = convertUnits(calcApparentExtractSG(ogUnit, fgUnit), Unit.SG).plato;
    const realExtract = calcRealExtract(ogUnit, apparentExtract);
    const apparentAttenuation = calcApparentAttenuation(ogUnit, apparentExtract);
    const realAttenuation = calcRealAttenuation(ogUnit, realExtract);
    const alcByWeight = calcAlcoholByWeight(ogUnit, realExtract);
    const alcByVolume = calcAlcoholByVolume(ogUnit, realExtract);
    return {
        apparentExtract,
        realExtract,
        apparentAttenuation,
        realAttenuation,
        alcByWeight,
        alcByVolume
    };
}

export const alcoholTerrillFormula = (og: Gravity, fg: Gravity): AlcoholCalculationResult => {
    const ogUnit = convertUnits(og.amount, og.unit);
    const fgUnit = convertUnits(fg.amount, fg.unit);
    const apparentExtract = convertUnits(calcApparentExtractSGTerrill(ogUnit, fgUnit), Unit.SG).plato;
    const realExtract = calcRealExtract(ogUnit, apparentExtract);
    const apparentAttenuation = calcApparentAttenuation(ogUnit, apparentExtract);
    const realAttenuation = calcRealAttenuation(ogUnit, realExtract);
    const alcByWeight = calcAlcoholByWeight(ogUnit, realExtract);
    const alcByVolume = calcAlcoholByVolume(ogUnit, realExtract);
    return {
        apparentExtract,
        realExtract,
        apparentAttenuation,
        realAttenuation,
        alcByWeight,
        alcByVolume
    };
}

export const alcoholStandardFormulaPlato = (og: Gravity, fg: Gravity): AlcoholCalculationResult => {
    const ogUnit = convertUnits(og.amount, og.unit);
    const fgUnit = convertUnits(fg.amount, fg.unit);
    const apparentExtract = fgUnit.plato;
    const realExtract = calcRealExtract(ogUnit, apparentExtract);
    const apparentAttenuation = calcApparentAttenuation(ogUnit, apparentExtract);
    const realAttenuation = calcRealAttenuation(ogUnit, realExtract);
    const alcByWeight = calcAlcoholByWeight(ogUnit, realExtract);
    const alcByVolume = calcAlcoholByVolume(ogUnit, realExtract);
    return {
        apparentExtract,
        realExtract,
        apparentAttenuation,
        realAttenuation,
        alcByWeight,
        alcByVolume
    };
}