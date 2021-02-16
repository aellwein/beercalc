
const brixToPlato = (brix) => {
    return brix / 1.03;
}

const platoToBrix = (plato) => {
    return plato * 1.03;
}

const platoToSg = (plato) => {
    return 1 + (plato / (258.6 - ((plato / 258.2) * 227.1)));
}

const sgToPlato = (sg) => {
    return 668.72 * sg - 463.37 - 205.347 * Math.pow(sg, 2);
}

const brixToOe = (brix) => {
    return brix * 4.25;
}

const oeToBrix = (oe) => {
    return oe / 4.25;
}

export const convertUnits = (amount, unit) => {
    switch (unit) {
        case 'brix':
            return { brix: amount, plato: brixToPlato(amount), sg: platoToSg(brixToPlato(amount)), oe: brixToOe(amount) };
        case 'plato':
            return { brix: platoToBrix(amount), plato: amount, sg: platoToSg(amount), oe:  brixToOe(platoToBrix(amount)) };
        case 'sg':
            return { brix: platoToBrix(sgToPlato(amount)), plato: sgToPlato(amount), sg: amount, oe: brixToOe(platoToBrix(sgToPlato(amount))) };
        case 'oe':
            return { brix: oeToBrix(amount), plato: brixToPlato(oeToBrix(amount)), sg: platoToSg(brixToPlato(oeToBrix(amount))), oe: amount };
        default:
            return {};
    }
}

const calcApparentExtractSG = (ogUnit, fgUnit) => {
    return 1.001843 - 0.002318474 * ogUnit.plato - 0.000007775 * Math.pow(ogUnit.plato, 2) -
        0.000000034 * Math.pow(ogUnit.plato, 3) + 0.00574 * fgUnit.brix +
        0.00003344 * Math.pow(fgUnit.brix, 2) +
        0.000000086 * Math.pow(fgUnit.brix, 3);
}

const calcApparentExtractSGTerrill = (ogUnit, fgUnit) => {
    return 1.0000 - 0.00085683 * ogUnit.plato + 0.0034941 * fgUnit.plato;
}

const calcRealExtract = (ogUnit, apparentExPlato) => {
    return 0.1808 * ogUnit.plato + 0.8192 * apparentExPlato;
}

const calcApparentAttenuation = (ogUnit, apparentExPlato) => {
    return (1 - apparentExPlato / ogUnit.plato) * 100;
}

const calcRealAttenuation = (ogUnit, realExPlato) => {
    return (1 - realExPlato / ogUnit.plato) * 100;
}

const calcAlcoholByWeight = (ogUnit, realExPlato) => {
    return (ogUnit.plato - realExPlato) / (2.0665 - 0.010665 * ogUnit.plato);
}

const calcAlcoholByVolume = (ogUnit, realExPlato) => {
    return 1 / 0.79 * (ogUnit.plato - realExPlato) / (2.0665 - 0.010665 * ogUnit.plato);
}

export const alcoholStandardFormula = (og, fg, unit) => {
    const ogUnit = convertUnits(og, unit);
    const fgUnit = convertUnits(fg, unit);
    const apparentExtract = convertUnits(calcApparentExtractSG(ogUnit, fgUnit), 'sg').plato;
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

export const alcoholTerrillFormula = (og, fg, unit) => {
    const ogUnit = convertUnits(og, unit);
    const fgUnit = convertUnits(fg, unit);
    const apparentExtract = convertUnits(calcApparentExtractSGTerrill(ogUnit, fgUnit), 'sg').plato;
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

export const alcoholStandardFormulaPlato = (og, fg, unit) => {
    const ogUnit = convertUnits(og, unit);
    const fgUnit = convertUnits(fg, unit);
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