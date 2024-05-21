import { useTranslation } from 'react-i18next';
import { BrowserRouter, Navigate, NavLink, Route, Routes } from 'react-router-dom';
import AlcoholCalc from '../alcohol-calc/AlcoholCalc';
import BrewhouseCalc from '../brewhouse-calc/BrewhouseCalc';
import ColorCalc from '../color-calc/ColorCalc';
import IbuCalc from '../ibu-calc/IbuCalc';
import LangPicker from '../lang-picker/LangPicker';
import ModeToggle from '../mode-toggle/ModeToggle';

const NavBar = () => {
  const { t } = useTranslation();

  return (
    <BrowserRouter basename={process.env.PUBLIC_URL || "/"}>
      <div className="flex flex-row flex-wrap gap-4 xs:gap-2 items-baseline">
        <NavLink to="/alcohol" className={({ isActive }) => "nav-link " + (isActive ? "text-indigo-600" : "hover:underline flex-shrink-0 text-gray-500")}>{t('alcohol calculator')}</NavLink>
        <NavLink to="/ibu" className={({ isActive }) => "nav-link " + (isActive ? "text-indigo-600" : "hover:underline flex-shrink-0 text-gray-500")}>{t('ibu calculator')}</NavLink>
        <NavLink to="/ebc" className={({ isActive }) => "nav-link " + (isActive ? "text-indigo-600" : "hover:underline flex-shrink-0 text-gray-500")}>{t('color calculator')}</NavLink>
        <NavLink to="/brewhouse" className={({ isActive }) => "nav-link " + (isActive ? "text-indigo-600" : "hover:underline flex-shrink-0 text-gray-500")}>{t('brewhouse calculator')}</NavLink>
        <div className="flex-grow flex-shrink-0">
          <div className="flex flex-row gap-4 items-baseline justify-end">
            <LangPicker />
            <ModeToggle />
          </div>
        </div>
      </div>
      <Routes>
        <Route path="/" element={<Navigate replace to="/alcohol" />} />
        <Route path="/alcohol" element={<AlcoholCalc />} />
        <Route path="/ibu" element={<IbuCalc />} />
        <Route path="/ebc" element={<ColorCalc />} />
        <Route path="/brewhouse" element={<BrewhouseCalc />} />
      </Routes>
    </BrowserRouter>
  );
}

export default NavBar;
