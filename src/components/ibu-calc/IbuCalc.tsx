import { useTranslation } from "react-i18next";
import { connect } from "react-redux";
import HopsAdder from "./HopsAdder";
import IbuPreset from "./IbuPreset";


const IbuCalc = () => {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col gap-4 dark:text-gray-400">
      <div className="text-2xl my-3">{t('ibu calculator')}</div>
      <IbuPreset />
      <HopsAdder />
    </div>
  );
}

const mapStateToProps = (rootState: any) => {
  return rootState.beerCalc;
}

export default connect(mapStateToProps, {})(IbuCalc);
