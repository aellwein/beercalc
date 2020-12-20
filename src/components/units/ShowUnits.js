import React from 'react';
import { useTranslation } from 'react-i18next';
import { convertUnits } from './calculations';

const calcUnits = (props) => {
    switch (props.type) {
        case 'og':
            return convertUnits(props.gravity.original, props.unit);
        case 'fg':
            return convertUnits(props.gravity.final, props.unit);
        default:
            return {};
    }
}

const ShowUnits = (props) => {
    const { t } = useTranslation();
    if (!props.type || !props.unit) {
        return <span></span>;
    }
    const units = calcUnits(props);

    switch (props.unit) {
        case 'brix':
            return <span>{units.brix.toFixed(1)} {t('brix')} / {units.plato.toFixed(1)} {t('plato')} / {units.sg.toFixed(3)} SG</span>;
        case 'plato':
            return <span>{units.plato.toFixed(1)} {t('plato')} / {units.brix.toFixed(1)} {t('brix')} / {units.sg.toFixed(3)} SG</span>;
        default:
            return <span></span>;
    }
}

export default ShowUnits;