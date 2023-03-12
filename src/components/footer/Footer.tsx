import { TFunction } from 'i18next';
import GitInfo, { GitInformation } from 'react-git-info/macro';
import { useTranslation } from 'react-i18next';

const baseUrl = "https://github.com/aellwein/beercalc";

const commit = (gitInfo: GitInformation, t: TFunction<"translation", undefined>) => {
    if (gitInfo.branch !== 'main') {
        return <span>{t('commit')} {gitInfo.commit.shortHash}</span>;
    } else {
        return <span>{t('commit')} <a className="text-indigo-600 hover:underline" href={baseUrl + "/commit/" + gitInfo.commit.hash}>{gitInfo.commit.shortHash}</a></span>;
    }
}

const Footer = () => {
    const gitInfo = GitInfo();
    const { t } = useTranslation();
    return (
        <footer className="my-6 text-center dark:text-gray-400">
            <a className="text-indigo-600 hover:underline" href={baseUrl}>{t('project on github')}</a> | &nbsp;
            {commit(gitInfo, t)} | &nbsp;
            {t('licensed under')} <a className="text-indigo-600 hover:underline" href="/license.html">{t('mit license')}</a>
        </footer >
    );
}

export default Footer;