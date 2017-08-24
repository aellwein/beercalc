import {BrowserModule} from '@angular/platform-browser';
import {NgModule} from '@angular/core';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {AlcoholCalculatorComponent} from './alcohol-calculator/alcohol-calculator.component';
import {FormsModule} from '@angular/forms';
import {CalcService} from './calc.service';

@NgModule({
  declarations: [
    AppComponent,
    AlcoholCalculatorComponent
  ],
  imports: [
    BrowserModule,
    FormsModule,
    AppRoutingModule,
  ],
  providers: [
    CalcService
  ],
  bootstrap: [AppComponent]
})
export class AppModule {
}
