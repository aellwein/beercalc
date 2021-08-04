import React from 'react';
import { useTranslation } from 'react-i18next';
import { BrowserRouter, NavLink, Redirect, Route, Switch } from 'react-router-dom';
import AlcoholCalc from '../alcohol-calc/AlcoholCalc';
import ColorCalc from '../color-calc/ColorCalc';
import IbuCalc from '../ibu-calc/IbuCalc';
import LangPicker from '../lang-picker/LangPicker';

const NavBar = () => {
    const { t } = useTranslation();

    return (
        <BrowserRouter>
            <nav>
                <div className="flex flex-row flex-wrap gap-10">
                        <NavLink to="/alcohol" className="hover:underline" activeClassName="underline text-indigo-600">{t('alcohol calculator')}</NavLink>
                        <NavLink to="/ibu" className="hover:underline" activeClassName="underline text-indigo-600">{t('ibu calculator')}</NavLink>
                        <NavLink to="/ebc" className="hover:underline" activeClassName="underline text-indigo-600">{t('color calculator')}</NavLink>
                        <div className="flex-grow flex-shrink-0 text-right">
                            <LangPicker />
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
            </Switch>
        </BrowserRouter>
    );
}

export default NavBar;