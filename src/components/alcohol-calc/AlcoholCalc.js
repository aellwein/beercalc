import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import CalcStandard from './CalcStandard';
import CalcTerrill from './CalcTerrill';
import GravityPicker from './GravityPicker';
import UnitPicker from './UnitPicker';


const columns = function* (props) {
    if (props.unit === 'brix') {
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12" key="calc-standard">
                <CalcStandard />
            </div>
        );
        yield (
            <div className="2xl:col-span-6 xl:col-span-6 lg:col-span-6 md:col-span-6 sm:col-span-12 xs:col-span-12 col-span-12" key="calc-terrill">
                <CalcTerrill />
            </div>
        );
    } else {
        yield (
            <div className="col-span-12" key="calc-standard">
                <CalcStandard />
            </div>
        );
    }
}

const AlcoholCalc = (props) => {
    const { t } = useTranslation();

    return (
        <div className="flex flex-col gap-4">
            <div className="text-2xl my-3">{t('alcohol calculator')}</div>
            <div><UnitPicker /></div>
            <div><GravityPicker /></div>
            <div className="grid grid-cols-12 gap-4">
                {[...columns(props)]}
            </div>
        </div>
    );
}

const mapStateToProps = (state) => {
    return state.beerCalc;
}

export default connect(mapStateToProps, {})(AlcoholCalc);