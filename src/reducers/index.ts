import { combineReducers } from "redux";
import { Action, ActionType } from "../action-types/ActionTypes";
import { CalculatorState, DisplayMode, HopsAddition, HopsForm, MaltAddition, MassUnit, Unit, VolumeMeasuredAt } from "../types";

const defaultState: CalculatorState = {
  displayMode: DisplayMode.Lite,
  unit: Unit.Brix,
  originalGravity: {
    amount: 12.0,
    unit: Unit.Brix,
  },
  finalGravity: {
    amount: 6.0,
    unit: Unit.Brix,
  },
  ibu: {
    boiling: 90,
    volume: 50,
    flameout: 5,
    flameoutTemp: 85,
    hops: [],
  },
  grain: {
    malt: [],
  },
  brewhouse: {
    volumeMeasuredAt: VolumeMeasuredAt.Hundred,
    grainMassFromBrewhouse: {
      originalGravity: {
        amount: 12,
        unit: Unit.Brix,
      },
      wortVolume: 20,
      volumeMeasuredAt: VolumeMeasuredAt.Hundred,
      brewhouseEfficiency: 65
    },
    grainMass: 5,
    grainMassUnit: MassUnit.Kilogram,
    wortVolume: 28,
  },
};

const beerCalc = (state: CalculatorState = defaultState, action: Action): CalculatorState => {
  switch (action.type) {
    case ActionType.ChangeUnit:
      return {
        ...state,
        unit: action.payload,
        originalGravity: { ...state.originalGravity, unit: action.payload },
        finalGravity: { ...state.finalGravity, unit: action.payload },
      };
    case ActionType.ChangeUnit2:
      return {
        ...state,
        brewhouse: {
          ...state.brewhouse,
          grainMassFromBrewhouse: {
            ...state.brewhouse.grainMassFromBrewhouse,
            originalGravity: {
              ...state.brewhouse.grainMassFromBrewhouse.originalGravity,
              unit: action.payload,
            },
          },
        },
      };
    case ActionType.SetOriginalGravity:
      return { ...state, originalGravity: action.payload };
    case ActionType.SetOriginalGravity2:
      return {
        ...state,
        brewhouse: {
          ...state.brewhouse,
          grainMassFromBrewhouse: {
            ...state.brewhouse.grainMassFromBrewhouse,
            originalGravity: action.payload
          }
        }
      };
    case ActionType.SetFinalGravity:
      return { ...state, finalGravity: action.payload };
    case ActionType.SetBoilingTime:
      // we have to check the hops additions here, to reduce the
      // boiling time of the particular hops addition to be not longer then the total boiling time.
      if (state.ibu.hops) {
        for (let h of state.ibu.hops) {
          h.boil = Math.min(h.boil, action.payload);
        }
      }
      return { ...state, ibu: { ...state.ibu, boiling: action.payload } };
    case ActionType.SetVolume:
      return { ...state, ibu: { ...state.ibu, volume: action.payload } };
    case ActionType.SetFlameOut:
      return { ...state, ibu: { ...state.ibu, flameout: action.payload } };
    case ActionType.SetFlameOutTemp:
      return { ...state, ibu: { ...state.ibu, flameoutTemp: action.payload } };
    case ActionType.NewHopsAddition:
      let ha: HopsAddition = {
        form: HopsForm.Whole,
        amount: 5.0,
        alpha: 7.0,
        boil: 90,
        ibu: 0
      };
      return { ...state, ibu: { ...state.ibu, hops: [...state.ibu.hops, ha] } };
    case ActionType.RemoveHopsAddition:
      state.ibu.hops.splice(action.payload, 1);
      return { ...state, ibu: { ...state.ibu, hops: [...state.ibu.hops] } };
    case ActionType.ChangeHopsAmount:
      let newHops = state.ibu.hops;
      newHops[action.payload.idx].amount = action.payload.amount;
      return { ...state, ibu: { ...state.ibu, hops: newHops } };
    case ActionType.ChangeHopsAlpha:
      let hops = state.ibu.hops;
      hops[action.payload.idx].alpha = action.payload.alpha;
      return { ...state, ibu: { ...state.ibu, hops } };
    case ActionType.ChangeHopsBoil:
      let h = state.ibu.hops;
      h[action.payload.idx].boil = action.payload.boil;
      return { ...state, ibu: { ...state.ibu, hops: h } };
    case ActionType.ChangeHopsForm:
      let ho = state.ibu.hops;
      ho[action.payload.idx].form = action.payload.form;
      return { ...state, ibu: { ...state.ibu, hops: ho } };
    case ActionType.NewMaltAddition:
      let malt: MaltAddition = { amount: 5.0, massUnit: MassUnit.Kilogram, color: 8 };
      return { ...state, grain: { ...state.grain, malt: [...state.grain.malt, malt] } };
    case ActionType.RemoveMaltAddition:
      state.grain.malt.splice(action.payload, 1);
      return { ...state, grain: { ...state.grain, malt: [...state.grain.malt] } };
    case ActionType.ChangeMaltMass:
      let newMalt = state.grain.malt;
      newMalt[action.payload.idx].amount = action.payload.mass;
      return { ...state, grain: { ...state.grain, malt: newMalt } };
    case ActionType.ChangeMaltMassUnit:
      let newMalt2 = state.grain.malt;
      newMalt2[action.payload.idx].massUnit = action.payload.massUnit;
      return { ...state, grain: { ...state.grain, malt: newMalt2 } };
    case ActionType.ChangeMaltColor:
      let newMalt3 = state.grain.malt;
      newMalt3[action.payload.idx].color = action.payload.color;
      return { ...state, grain: { ...state.grain, malt: newMalt3 } };
    case ActionType.TurnDarkMode:
      return { ...state, displayMode: DisplayMode.Dark };
    case ActionType.TurnLiteMode:
      return { ...state, displayMode: DisplayMode.Lite };
    case ActionType.SetGrainMassAndUnit:
      return { ...state, brewhouse: { ...state.brewhouse, grainMass: action.payload.grainMass, grainMassUnit: action.payload.grainMassUnit } };
    case ActionType.ChangeWortVolume:
      return { ...state, brewhouse: { ...state.brewhouse, wortVolume: action.payload } };
    case ActionType.ChangeWortVolume2:
      return { ...state, brewhouse: { ...state.brewhouse, grainMassFromBrewhouse: { ...state.brewhouse.grainMassFromBrewhouse, wortVolume: action.payload } } };
    case ActionType.ChangeVolumeMeasuredAt:
      return { ...state, brewhouse: { ...state.brewhouse, volumeMeasuredAt: action.payload } };
    case ActionType.ChangeVolumeMeasuredAt2:
      return { ...state, brewhouse: { ...state.brewhouse, grainMassFromBrewhouse: { ...state.brewhouse.grainMassFromBrewhouse, volumeMeasuredAt: action.payload } } };
    case ActionType.ChangeBrewhouseEfficiency:
      return { ...state, brewhouse: { ...state.brewhouse, grainMassFromBrewhouse: { ...state.brewhouse.grainMassFromBrewhouse, brewhouseEfficiency: action.payload } } };
    default:
      return state;
  }
}

export default combineReducers({ beerCalc });
