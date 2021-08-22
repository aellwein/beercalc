import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { finalGravity, originalGravity } from '../../actions';
import ShowUnits from '../units/ShowUnits';

const onChangeOg = (props, newOg) => {
    let og = parseFloat(newOg);
    if (isNaN(og) || og === null) {
        return;
    }
    props.originalGravity(og);
}

const onChangeFg = (props, newFg) => {
    let fg = parseFloat(newFg);
    if (isNaN(fg) || fg === null) {
        return;
    }
    props.finalGravity(fg);
}

const GravityPicker = (props) => {
    const { t } = useTranslation();
    return (
        <div className="grid grid-cols-12 gap-3 shadow-md p-4 items-baseline">
            <div className="2xl:col-span-2 xl:col-span-2 lg:col-span-3 md:col-span-2 sm:col-span-6 col-span-12 xs:col-span-12 2xl:text-right xl:text-right lg:text-right md:text-right sm:text-left xs:text-left text-left">
                <span>{t('original gravity')}</span>
            </div>
            <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 md:col-span-4 sm:col-span-6 col-span-12 xs:col-span-12">
                <input className="border-gray-300 p-1 border-solid border-1 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                    type="number" id="og" min=".1" max="40" step=".1" value={props.gravity.original} onChange={e => onChangeOg(props, e.target.value)}></input>
            </div>
            <div className="2xl:col-span-8 xl:col-span-7 lg:col-span-6 md:col-span-6 sm:col-span-12 col-span-12 xs:col-span-12 2xl:text-left xl:text-left lg:text-left sm:text-right">
                <ShowUnits type="og" unit={props.unit} gravity={props.gravity} />
            </div>


            <div className="2xl:col-span-2 xl:col-span-2 lg:col-span-3 md:col-span-2 sm:col-span-6 col-span-12 xs:col-span-12 md:text-right sm:text-left xs:text-left text-left">
                <span>{t('final gravity')}</span>
            </div>

            <div className="2xl:col-span-2 xl:col-span-3 lg:col-span-3 sm:col-span-6 md:col-span-4 col-span-12 xs:col-span-12">
                <input className="border-gray-300 p-1 border-solid border-1 flex-shrink flex-grow-0 focus:border-blue-300 focus:ring outline-none dark:bg-gray-700 dark:text-gray-300"
                    type="number" id="fg" min=".1" max="40" step=".1" value={props.gravity.final} onChange={e => onChangeFg(props, e.target.value)}></input>
            </div>
            <div className="2xl:col-span-8 xl:col-span-7 lg:col-span-6 md:col-span-6 sm:col-span-12 col-span-12 xs:col-span-12 xl:text-left 2xl:text-left lg:text-left sm:text-right">
                <ShowUnits type="fg" unit={props.unit} gravity={props.gravity} />
            </div>
        </div>
    );
}

const mapStateToProps = (state) => {
    return { gravity: state.beerCalc.gravity, unit: state.beerCalc.unit };
}

export default connect(mapStateToProps, { originalGravity, finalGravity })(GravityPicker);