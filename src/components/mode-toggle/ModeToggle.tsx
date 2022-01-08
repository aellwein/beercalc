import { useTranslation } from 'react-i18next';
import { connect, ConnectedProps } from 'react-redux';
import { turnDarkMode, turnLiteMode } from '../../actions';
import { CalculatorState, DisplayMode } from '../../types';

interface ModeToggleProps extends PropsFromRedux {
}

const toggleMode = (props: ModeToggleProps) => {
    if (props.displayMode === DisplayMode.Dark) {
        props.turnLiteMode();
    } else {
        props.turnDarkMode();
    }
}

const ModeToggle: React.FC<ModeToggleProps> = (props: ModeToggleProps) => {
    const { t } = useTranslation();
    if (typeof (props.displayMode) === 'undefined') {
        return <div></div>;
    }
    if (props.displayMode === DisplayMode.Lite) {
        return <button className="rounded-3xl border-1 border-solid border-gray-300 p-1 dark:bg-gray-700 dark:text-gray-300" title={t('light mode')} onClick={() => toggleMode(props)}>🌞</button>;
    } else if (props.displayMode === DisplayMode.Dark) {
        return <button className="rounded-3xl border-1 border-solid border-gray-300 p-1 dark:bg-gray-700 dark:text-gray-300" title={t('dark mode')} onClick={() => toggleMode(props)}>🌙</button>;
    } else {
        return <div>error: invalid mode!</div>
    }
}

const mapStateToProps = (rootState: any) => {
    const state: CalculatorState = rootState.beerCalc;
    const displayMode = state.displayMode;
    switch (displayMode) {
        case DisplayMode.Dark:
            document.documentElement.classList.add('dark');
            break;
        case DisplayMode.Lite:
            document.documentElement.classList.remove('dark');
            break;
    }
    return { displayMode };
}

const connector = connect(mapStateToProps, { turnDarkMode, turnLiteMode });
type PropsFromRedux = ConnectedProps<typeof connector>;
export default connector(ModeToggle);