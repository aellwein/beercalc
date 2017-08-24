import {Component, OnInit} from '@angular/core';
import {CalcResultBrix, CalcResultPlato, CalcService} from '../calc.service';

@Component({
  selector: 'app-alcohol-calculator',
  templateUrl: './alcohol-calculator.component.html',
  styleUrls: ['./alcohol-calculator.component.scss']
})
export class AlcoholCalculatorComponent implements OnInit {
  unit = 'brix';
  stammwuerze = 12;
  restextrakt = 6;
  calcResultBrix: CalcResultBrix;
  calcResultPlato: CalcResultPlato;

  constructor(private calcService: CalcService) {
  }

  ngOnInit() {
    this.calculate();
  }

  validInput(): boolean {
    return (this.restextrakt < this.stammwuerze) &&
      (this.stammwuerze >= 0.1 && this.stammwuerze <= 35) &&
      (this.restextrakt >= 0.1 && this.restextrakt <= 35);
  }

  einheit(): string {
    return this.unit === 'brix' ? 'Brix' : '°P';
  }

  displayBrixCalculations(): boolean {
    return this.unit === 'brix' &&
      this.validInput() &&
      this.calcResultBrix &&
      this.calcResultBrix.standard.SRE_Plato >= 0.0 &&
      this.calcResultBrix.terrill.SRE_Plato >= 0.0 &&
      this.calcResultBrix.standard.sEVG > 0 &&
      this.calcResultBrix.terrill.sEVG > 0 &&
      this.calcResultBrix.standard.tEVG > 0 &&
      this.calcResultBrix.terrill.tEVG > 0;
  }

  displayPlatoCalculations(): boolean {
    return this.unit === 'plato' &&
      this.validInput() &&
      this.calcResultPlato &&
      this.calcResultPlato.re.plato >= 0.0 &&
      this.calcResultPlato.formula.sEVG >= 0.0 &&
      this.calcResultPlato.formula.tEVG > 0;
  }

  calculate(): void {
    if (!this.validInput()) {
      return;
    }
    if (this.unit === 'brix') {
      this.calcResultBrix = this.calcService.calculateFromBrix(this.stammwuerze, this.restextrakt);
    } else {
      this.calcResultPlato = this.calcService.calculateFromPlato(this.stammwuerze, this.restextrakt);
    }
  }
}
