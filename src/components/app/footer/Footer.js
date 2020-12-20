import React from 'react';
import { useTranslation } from 'react-i18next';
import './Footer.css';
import GitInfo from 'react-git-info/macro';

const Footer = () => {
    const { t } = useTranslation();
    const baseUrl = "https://github.com/aellwein/beercalc";
    const gitInfo = GitInfo();
    return (
        <footer className="footer-thin">
            <div className="content has-text-centered">
                <a href={baseUrl}>{t('project on github')}</a> | &nbsp;
                {t('version')} <a href={baseUrl + "/releases/tag/" + gitInfo.tags[0]}>{gitInfo.tags[0]}</a> | &nbsp;
                {t('commit')} <a href={baseUrl + "/commit/" + gitInfo.commit.hash}>{gitInfo.commit.shortHash}</a> | &nbsp;
            {t('licensed under')} <a href="/license.html">{t('mit license')}</a>
            </div>
        </footer >
    );
}

export default Footer;