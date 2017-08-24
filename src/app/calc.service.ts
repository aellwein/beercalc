import {Injectable} from '@angular/core';

export class CalcUnits {
  brix: number;
  plato: number;
  sg: number;
}

export class FormulaResult {
  SRE_SG: number;
  SRE_Plato: number;
  TRE: number;
  sEVG: number;
  tEVG: number;
  alcGw: number;
  alcVol: number;
}

export class CalcResultPlato {
  stw: CalcUnits;
  re: CalcUnits;
  formula: FormulaResult;
}

export class CalcResultBrix {
  // Stammwürze und Restextrakt
  stw: CalcUnits;
  re: CalcUnits;
  // Berechnungsergebnisse nach Standard- und Terrill-Formeln
  standard: FormulaResult;
  terrill: FormulaResult;
}

@Injectable()
export class CalcService {

  constructor() {
  }

  private static brixToPlato(brix: number): number {
    return brix / 1.03;
  }

  private static platoToBrix(plato: number): number {
    return plato * 1.03;
  }

  private static platoToSG(plato: number): number {
    return 1 + (plato / (258.6 - ((plato / 258.2) * 227.1)));
  }

  private static specificGravityToPlato(sg: number): number {
    return 668.72 * sg - 463.37 - 205.347 * Math.pow(sg, 2);
  }

  private static getSRE(stw: CalcUnits, re: CalcUnits): number {
    return 1.001843 - 0.002318474 * stw.plato - 0.000007775 * Math.pow(stw.plato, 2) -
      0.000000034 * Math.pow(stw.plato, 3) + 0.00574 * re.brix +
      0.00003344 * Math.pow(re.brix, 2) +
      0.000000086 * Math.pow(re.brix, 3);
  }

  private static getSREnachTerrill(stw: CalcUnits, re: CalcUnits): number {
    return 1.0000 - 0.00085683 * stw.plato + 0.0034941 * re.plato;
  }

  private static brixToCalcUnits(brix: number): CalcUnits {
    const btp = CalcService.brixToPlato(brix);
    return {
      brix: brix,
      plato: btp,
      sg: CalcService.platoToSG(btp)
    } as CalcUnits;
  }

  private static platoToCalcUnits(plato: number): CalcUnits {
    return {
      plato: plato,
      brix: CalcService.platoToBrix(plato),
      sg: CalcService.platoToSG(plato)
    } as CalcUnits;
  }

  private static getTRE(stw: CalcUnits, srePlato: number): number {
    return 0.1808 * stw.plato + 0.8192 * srePlato;
  }

  private static getSEVGPercent(stw: CalcUnits, srePlato: number): number {
    return (1 - srePlato / stw.plato) * 100;
  }

  private static getTEVGPercent(stw: CalcUnits, trePlato: number): number {
    return (1 - trePlato / stw.plato) * 100;
  }

  private static getAlcGewPercent(stw: CalcUnits, trePlato: number): number {
    return (stw.plato - trePlato) / (2.0665 - 0.010665 * stw.plato);
  }

  private static getAlcVolPercent(stw: CalcUnits, trePlato: number): number {
    return 1 / 0.79 * (stw.plato - trePlato) / (2.0665 - 0.010665 * stw.plato);
  }

  private static calculateStandard(stw: CalcUnits, re: CalcUnits): FormulaResult {
    const sreSG = CalcService.getSRE(stw, re);
    const srePlato = CalcService.specificGravityToPlato(sreSG);
    const tre = CalcService.getTRE(stw, srePlato);
    const sevg = CalcService.getSEVGPercent(stw, srePlato);
    const tevg = CalcService.getTEVGPercent(stw, tre);
    const alcGw = CalcService.getAlcGewPercent(stw, tre);
    const alcVol = CalcService.getAlcVolPercent(stw, tre);
    return {
      SRE_SG: sreSG,
      SRE_Plato: srePlato,
      TRE: tre,
      sEVG: sevg,
      tEVG: tevg,
      alcGw: alcGw,
      alcVol: alcVol
    } as FormulaResult;
  }

  private static calculateTerrill(stw: CalcUnits, re: CalcUnits): FormulaResult {
    const sreSG = CalcService.getSREnachTerrill(stw, re);
    const srePlato = CalcService.specificGravityToPlato(sreSG);
    const tre = CalcService.getTRE(stw, srePlato);
    const sevg = CalcService.getSEVGPercent(stw, srePlato);
    const tevg = CalcService.getTEVGPercent(stw, tre);
    const alcGw = CalcService.getAlcGewPercent(stw, tre);
    const alcVol = CalcService.getAlcVolPercent(stw, tre);
    return {
      SRE_SG: sreSG,
      SRE_Plato: srePlato,
      TRE: tre,
      sEVG: sevg,
      tEVG: tevg,
      alcGw: alcGw,
      alcVol: alcVol
    } as FormulaResult;
  }

  public calculateFromBrix(stwBrix: number, reBrix: number): CalcResultBrix {
    const stwCalcUnits = CalcService.brixToCalcUnits(stwBrix);
    const reCalcUnits = CalcService.brixToCalcUnits(reBrix);
    return {
      stw: stwCalcUnits,
      re: reCalcUnits,
      standard: CalcService.calculateStandard(stwCalcUnits, reCalcUnits),
      terrill: CalcService.calculateTerrill(stwCalcUnits, reCalcUnits)
    } as CalcResultBrix;
  }

  public calculateFromPlato(stwPlato: number, rePlato: number): CalcResultPlato {
    const stwCalcUnits = CalcService.platoToCalcUnits(stwPlato);
    const reCalcUnits = CalcService.platoToCalcUnits(rePlato);
    const tre = CalcService.getTRE(stwCalcUnits, rePlato);
    const sevg = CalcService.getSEVGPercent(stwCalcUnits, rePlato);
    const tevg = CalcService.getTEVGPercent(stwCalcUnits, tre);
    const alcGw = CalcService.getAlcGewPercent(stwCalcUnits, tre);
    const alcVol = CalcService.getAlcVolPercent(stwCalcUnits, tre);
    return {
      stw: stwCalcUnits,
      re: reCalcUnits,
      formula: {
        SRE_Plato: rePlato,
        SRE_SG: CalcService.platoToSG(rePlato),
        TRE: tre,
        sEVG: sevg,
        tEVG: tevg,
        alcGw: alcGw,
        alcVol: alcVol
      } as FormulaResult
    };
  }
}
