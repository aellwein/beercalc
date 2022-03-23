import { useTranslation } from 'react-i18next';
import { Gravity, Unit } from '../../types';
import { convertUnits } from './calculations';

interface ShowUnitsProps {
    gravity: Gravity;
}

const ShowUnits = (props: ShowUnitsProps) => {
    const { t } = useTranslation();
    if (!props.gravity) {
        return <span></span>;
    }
    const units = convertUnits(props.gravity.amount, props.gravity.unit);

    switch (props.gravity.unit) {
        case Unit.Brix:
            return <span>{units.brix.toFixed(1)} {t('brix')} / {units.plato.toFixed(1)} {t('plato')} / {units.oe.toFixed(1)} {t('oechsle')} / {units.sg.toFixed(3)} SG</span>;
        case Unit.Plato:
            return <span>{units.plato.toFixed(1)} {t('plato')} / {units.brix.toFixed(1)} {t('brix')} / {units.oe.toFixed(1)} {t('oechsle')} / {units.sg.toFixed(3)} SG</span>;
        default:
            return <span></span>;
    }
}

export default ShowUnits;