import React from 'react';
import { useTranslation } from 'react-i18next';
import { BrowserRouter, NavLink, Redirect, Route, Switch } from 'react-router-dom';
import AlcoholCalc from '../alcohol-calc/AlcoholCalc';
import BrewhouseEfficiencyCalc from '../brewhouse-calc/BrewhouseCalc';
import ColorCalc from '../color-calc/ColorCalc';
import IbuCalc from '../ibu-calc/IbuCalc';
import LangPicker from '../lang-picker/LangPicker';
import ModeToggle from '../mode-toggle/ModeToggle';

const NavBar = () => {
    const { t } = useTranslation();

    return (
        <BrowserRouter>
            <nav>
                <div className="flex flex-row flex-wrap gap-8 xs:gap-2 items-baseline">
                    <NavLink to="/alcohol" className="hover:underline flex-shrink-0 text-gray-500" activeClassName="underline text-indigo-600">{t('alcohol calculator')}</NavLink>
                    <NavLink to="/ibu" className="hover:underline flex-shrink-0 text-gray-500" activeClassName="underline text-indigo-600">{t('ibu calculator')}</NavLink>
                    <NavLink to="/ebc" className="hover:underline flex-shrink-0 text-gray-500" activeClassName="underline text-indigo-600">{t('color calculator')}</NavLink>
                    <NavLink to="/brewhouse" className="hover:underline flex-shrink-0 text-gray-500" activeClassName="underline text-indigo-600">{t('brewhouse calculator')}</NavLink>
                    <div className="flex-grow flex-shrink-0">
                        <div className="flex flex-row gap-3 items-baseline justify-end">
                            <LangPicker />
                            <ModeToggle />
                        </div>
                    </div>
                </div>
            </nav>
            <Switch>
                <Redirect exact from="/" to="/alcohol" />
                <Route path="/alcohol">
                    <AlcoholCalc />
                </Route>
                <Route path="/ibu">
                    <IbuCalc />
                </Route>
                <Route path="/ebc">
                    <ColorCalc />
                </Route>
                <Route path="/brewhouse">
                    <BrewhouseEfficiencyCalc />
                </Route>
            </Switch>
        </BrowserRouter>
    );
}

export default NavBar;