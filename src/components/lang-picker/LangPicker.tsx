import { i18n } from "i18next";
import { ReactElement } from "react";
import { useTranslation } from "react-i18next";

const languages = function* (): Generator<ReactElement, void, undefined> {
  const langs = [
    { 'short': 'en', 'long': '🇬🇧 EN' },
    { 'short': 'de', 'long': '🇩🇪 DE' },
    { 'short': 'ru', 'long': '🇷🇺 РУ' },
  ];
  for (let i of langs) {
    yield <option key={i.short} value={i.short}>{i.long}</option>
  }
}

const onChangeLang = (lang: string, i18n: i18n) => {
  i18n.changeLanguage(lang);
}

const getLang = (lang: string | undefined): string => {
  if (typeof (lang) === 'undefined') {
    return 'en';
  }
  if (lang.indexOf("-") === -1) {
    return lang;
  }
  return lang.split('-')[0].toLowerCase();
}

const LangPicker = () => {
  const { i18n } = useTranslation();

  return (
    <div>
      <select className="appearance-none rounded-none bg-white border-gray-300 p-1 border-solid border-1 dark:bg-gray-700 dark:text-gray-300"
        onChange={e => onChangeLang(e.target.value, i18n)}
        value={getLang(i18n.resolvedLanguage)}>
        {[...languages()]}
      </select>
    </div>
  );
}

export default LangPicker;
