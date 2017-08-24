import {async, ComponentFixture, TestBed} from '@angular/core/testing';

import {AlcoholCalculatorComponent} from './alcohol-calculator.component';

describe('AlcoholCalculatorComponent', () => {
  let component: AlcoholCalculatorComponent;
  let fixture: ComponentFixture<AlcoholCalculatorComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ AlcoholCalculatorComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AlcoholCalculatorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should be created', () => {
    expect(component).toBeTruthy();
  });
});
