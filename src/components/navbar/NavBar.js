import React from 'react';
import { useTranslation } from 'react-i18next';
import { BrowserRouter, NavLink, Redirect, Route, Switch } from 'react-router-dom';
import AlcoholCalc from '../alcohol-calc/AlcoholCalc';
import IbuCalc from '../ibu-calc/IbuCalc';
import LangPicker from '../lang-picker/LangPicker';

const NavBar = () => {
    const { t } = useTranslation();

    const onBurgerClick = () => {
        const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);
        if ($navbarBurgers.length > 0) {
            $navbarBurgers.forEach(el => {
                el.addEventListener('click', () => {
                    const target = el.dataset.target;
                    const $target = document.getElementById(target);
                    el.classList.toggle('is-active');
                    $target.classList.toggle('is-active');
                });
            });
        }
    }

    return (
        <BrowserRouter>
            <nav className="navbar" role="navigation" aria-label="main navigation">
                <div className="navbar-burger" style={{ border: "0", backgroundColor: "white" }} aria-label="menu" aria-expanded="false" data-target="navbarMenu" onClick={onBurgerClick}>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </div>
                <div id="navbarMenu" className="navbar-menu">
                    <div className="navbar-start">
                        <NavLink to="/alcohol" className="navbar-item" activeClassName="is-tab is-active">{t('alcohol calculator')}</NavLink>
                        <NavLink to="/ibu" className="navbar-item" activeClassName="is-tab is-active">{t('ibu calculator')}</NavLink>
                    </div>
                    <div className="navbar-end">
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
            </Switch>
        </BrowserRouter>
    );
}

export default NavBar;