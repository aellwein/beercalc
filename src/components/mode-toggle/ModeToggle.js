import React from 'react';
import { useTranslation } from 'react-i18next';
import { connect } from 'react-redux';
import { turnDarkMode, turnLiteMode } from '../../actions';

const toggleMode = (props) => {
    if (props.displayMode === 'dark') {
        props.turnLiteMode();
    } else {
        props.turnDarkMode();
    }
}

const ModeToggle = (props) => {
    const { t } = useTranslation();
    if (typeof (props.displayMode) === 'undefined') {
        return <div></div>;
    }
    if (props.displayMode === 'lite') {
        return <button title={t('turn dark mode')} onClick={() => toggleMode(props)}>🌛</button>;
    } else if (props.displayMode === 'dark') {
        return <button title={t('turn light mode')} onClick={() => toggleMode(props)}>🌞</button>;
    } else {
        return <div>error: invalid mode!</div>
    }
}

const mapStateToProps = (state, _) => {
    const displayMode = state.beerCalc.displayMode;
    switch (displayMode) {
        case 'dark':
            document.documentElement.classList.add('dark');
            break;
        case 'lite':
            document.documentElement.classList.remove('dark');
            break;
        default:
            break;
    }
    return { displayMode };
}

export default connect(mapStateToProps, { turnDarkMode, turnLiteMode })(ModeToggle);