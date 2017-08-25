import {BrowserModule, Title} from '@angular/platform-browser';
import {NgModule} from '@angular/core';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {AlcoholCalculatorComponent} from './alcohol-calculator/alcohol-calculator.component';
import {FormsModule} from '@angular/forms';
import {CalcService} from './calc.service';
import {TranslationService} from './translation.service';
import {HttpModule} from '@angular/http';

@NgModule({
  declarations: [
    AppComponent,
    AlcoholCalculatorComponent
  ],
  imports: [
    BrowserModule,
    FormsModule,
    AppRoutingModule,
    HttpModule
  ],
  providers: [
    CalcService,
    TranslationService,
    HttpModule,
    Title
  ],
  bootstrap: [AppComponent]
})
export class AppModule {
}
