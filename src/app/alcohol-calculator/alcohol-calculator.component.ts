import {Component, OnInit} from '@angular/core';
import {CalcResultBrix, CalcResultPlato, CalcService} from '../calc.service';
import {Language, TranslationService} from '../translation.service';

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
  selectedLanguage = 'de';
  languages: Language[];

  constructor(private calcService: CalcService,
              private translationService: TranslationService) {
    this.languages = translationService.getLanguages();
    this.selectedLanguage = this.translationService.getLanguage();
  }

  ngOnInit() {
    this.restoreState();
    this.calculate();
  }

  private restoreState(): void {
    if (localStorage) {
      if (localStorage.selectedLanguage) {
        this.selectedLanguage = localStorage.getItem('selectedLanguage');
        this.translationService.setLanguage(this.selectedLanguage);
      }
      if (localStorage.stammwuerze) {
        this.stammwuerze = parseFloat(localStorage.getItem('stammwuerze'));
      }
      if (localStorage.restextrakt) {
        this.restextrakt = parseFloat(localStorage.getItem('restextrakt'));
      }
      if (localStorage.unit) {
        this.unit = localStorage.getItem('unit');
      }
    }
  }


  _(key: string): string {
    return this.translationService._(key);
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

  changeLang(): void {
    this.translationService.setLanguage(this.selectedLanguage);
    if (localStorage) {
      localStorage.setItem('selectedLanguage', this.selectedLanguage);
    }
  }

  changeUnit(): void {
    if (this.validInput()) {
      this.calculate();
    }
  }

  calculate(): void {
    if (!this.validInput()) {
      return;
    }
    if (localStorage) {
      localStorage.setItem('stammwuerze', this.stammwuerze.toString());
      localStorage.setItem('restextrakt', this.restextrakt.toString());
      localStorage.setItem('unit', this.unit);
    }

    if (this.unit === 'brix') {
      this.calcResultBrix = this.calcService.calculateFromBrix(this.stammwuerze, this.restextrakt);
    } else {
      this.calcResultPlato = this.calcService.calculateFromPlato(this.stammwuerze, this.restextrakt);
    }
  }
}
