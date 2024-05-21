import { TFunction } from 'i18next';
import { useTranslation } from "react-i18next";
import { connect, ConnectedProps } from "react-redux";
import { changeHopsAlpha, changeHopsAmount, changeHopsBoil, changeHopsForm, newHopsAddition, removeHopsAddition } from '../../actions';
import { CalculatorState, HopsForm } from "../../types";
import { convertUnits } from "../units/calculations";

interface IbuCalcProps extends PropsFromRedux {
}

const onAddHopsClick = (props: IbuCalcProps) => {
  props.newHopsAddition();
}

const onRemoveHopsClick = (props: IbuCalcProps, idx: number) => {
  props.removeHopsAddition(idx);
}

const onChangeAmount = (props: IbuCalcProps, idx: number, amount: string) => {
  let a = parseFloat(amount);
  if (isNaN(a) || a === null) {
    return;
  }
  props.changeHopsAmount(idx, a);
}

const onChangeAlpha = (props: IbuCalcProps, idx: number, newAlpha: string) => {
  let a = parseFloat(newAlpha);
  if (isNaN(a) || a === null) {
    return;
  }
  props.changeHopsAlpha(idx, a);
}

const onChangeBoil = (props: IbuCalcProps, idx: number, boilTime: string) => {
  let b = parseFloat(boilTime);
  if (isNaN(b) || b === null) {
    return;
  }
  if (b > Math.min(b, props.ibu.boiling)) {
    b = Math.min(b, props.ibu.boiling)
  }
  props.changeHopsBoil(idx, b);
}

const onChangeForm = (props: IbuCalcProps, idx: number, form: HopsForm) => {
  props.changeHopsForm(idx, form);
}

const hopsAdditions = function*(props: IbuCalcProps, t: TFunction<"translation", undefined>) {
  for (let i = 0; i < props.ibu.hops.length; i++) {
    let a = props.ibu.hops[i];
    yield (
      <div className='flex flex-row items-baseline gap-4' key={"ha-" + i}>
        <div className="underline text-right" key={i + "-col1-1"}>{t('hops addition') + ' #' + (i + 1)}</div>
        <button className="py-2 px-2 bg-red-300 text-white hover:bg-red-600 p-1" onClick={() => onRemoveHopsClick(props, i)}>X</button>
      </div>
    );
    yield (
      <div className='flex flex-row items-baseline gap-4' key={"form-" + i}>
        <div key={i + "-col2-1"} className="">{t('form')}</div>
        <select className="p-1 appearance-none rounded-none bg-white border-gray-300 border-1 border-solid dark:bg-gray-700 dark:text-gray-300" defaultValue="whole" onChange={(e) => onChangeForm(props, i, e.target.value as HopsForm)}>
          <option value="whole">{t('whole')}</option>
          <option value="plugs">{t('plugs')}</option>
        </select>
      </div>
    );
    yield (
      <div className='flex flex-row items-baseline gap-4' key={"amount-" + i}>
        <div>{t('amount')}</div>
        <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="0" max="5000" step=".1" value={a.amount} onChange={(e) => onChangeAmount(props, i, e.target.value)}></input>
      </div>
    );
    yield (
      <div className='flex flex-row items-baseline gap-4' key={"acid-" + i}>
        <div>{t('alpha acid')}</div>
        <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="0" max="90" step=".1" value={a.alpha} onChange={(e) => onChangeAlpha(props, i, e.target.value)}></input>
      </div>
    );
    yield (
      <div className='flex flex-row items-baseline gap-4' key={"boiltime-" + i}>
        <div>{t('boil time')}</div>
        <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300" type="number" min="0" max="600" value={a.boil} onChange={(e) => onChangeBoil(props, i, e.target.value)}></input>
      </div>
    );
    yield (
      <div className='flex flex-row items-baseline gap-4' key={"bitterness-" + i}>
        <div className="grow">{t('bitterness')}</div>
        <div className="text-right dark:text-gray-200">{a.ibu} IBU</div>
      </div>
    )
  }
}

const calcBitterness = (props: IbuCalcProps): number => {
  let b = 0.0;
  if (props.ibu.hops.length === 0) {
    return b;
  }
  for (let h of props.ibu.hops) {
    b += h.ibu;
  }
  return Number.parseFloat(b.toFixed(1));
}

const HopsAdder = (props: IbuCalcProps) => {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col gap-4 p-4 shadow-md">
      {[...hopsAdditions(props, t)]}
      <div className="flex flex-row gap-4">
        <div className="grow dark:text-gray-200"><strong>{t('total bitterness')}</strong></div>
        <div className="text-right dark:text-gray-200"><strong>{calcBitterness(props)} IBU</strong></div>
      </div>
      <div className="mr-auto">
        <button className="py-2 px-4 text-white bg-green-500 rounded-lg shadow-md hover:bg-green-700" onClick={() => onAddHopsClick(props)}>{t('add hops')}</button>
      </div>
    </div>
  );
}

const recalcIbu = (state: CalculatorState) => {
  const og = convertUnits(state.originalGravity.amount, state.originalGravity.unit);
  // isomerization speed factor + time
  const isoSpeedFactor = 0.046 * Math.exp(0.031 * state.ibu.flameoutTemp);
  const addIsoTime = isoSpeedFactor * state.ibu.flameout;
  for (let h of state.ibu.hops) {
    // better alpha acid isomerization through pellets
    const hopsFactor = (h.form === HopsForm.Whole) ? 1.1 : 1.0;
    const ibu = (hopsFactor * ((h.amount * h.alpha * 10) / state.ibu.volume) * (1.65 * Math.pow(0.000125, (0.004 * og.plato))) * ((1 - Math.exp(-0.04 * (h.boil + addIsoTime))) / 4.15)).toFixed(1);
    h.ibu = Number.parseFloat(ibu);
  }
}

const mapStateToProps = (rootState: any) => {
  let state: CalculatorState = rootState.beerCalc;
  recalcIbu(state);
  return state;
}

const connector = connect(mapStateToProps, { newHopsAddition, removeHopsAddition, changeHopsAmount, changeHopsForm, changeHopsAlpha, changeHopsBoil });
type PropsFromRedux = ConnectedProps<typeof connector>;

export default connector(HopsAdder);
