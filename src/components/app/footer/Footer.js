import React from 'react';
import { useTranslation } from 'react-i18next';
import './Footer.css';

const Footer = () => {
    const { t } = useTranslation();
    return (
        <footer className="footer-thin">
            <div className="content has-text-centered">
                <a href="https://github.com/aellwein/beercalc">{t('project on github')}</a> | {t('licensed under')} <a href="/license.html">{t('mit license')}</a>
            </div>
        </footer>
    );
}

export default Footer;