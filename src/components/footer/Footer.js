import React from 'react';
import GitInfo from 'react-git-info/macro';
import { useTranslation } from 'react-i18next';

const baseUrl = "https://github.com/aellwein/beercalc";


const version = (gitInfo, t) => {
    if (gitInfo.branch !== 'master' || gitInfo.tags.length < 1) {
        return <span>{t('developer version')}</span>;
    } else {
        return <span>{t('version')} <a className="text-indigo-600 hover:underline" href={baseUrl + "/releases/tag/" + gitInfo.tags[0]}>{gitInfo.tags[0]}</a></span>
    }
}

const commit = (gitInfo, t) => {
    if (gitInfo.branch !== 'master' || gitInfo.tags.length < 1) {
        return <span>{t('commit')} {gitInfo.commit.shortHash}</span>;
    } else {
        return <span>{t('commit')} <a className="text-indigo-600 hover:underline" href={baseUrl + "/commit/" + gitInfo.commit.hash}>{gitInfo.commit.shortHash}</a></span>;
    }
}

const Footer = () => {
    const gitInfo = GitInfo();
    const { t } = useTranslation();
    return (
        <footer className="my-6 text-center">
            <a className="text-indigo-600 hover:underline" href={baseUrl}>{t('project on github')}</a> | &nbsp;
            {version(gitInfo, t)} | &nbsp;
            {commit(gitInfo, t)} | &nbsp;
            {t('licensed under')} <a className="text-indigo-600 hover:underline" href="/license.html">{t('mit license')}</a>
        </footer >
    );
}

export default Footer;