import React from 'react';
import { useTranslation } from "react-i18next";

const languages = function* () {
    const langs = [
        { 'short': 'en', 'long': '🇺🇸 EN' },
        { 'short': 'de', 'long': '🇩🇪 DE' },
        { 'short': 'ru', 'long': '🇷🇺 РУ' },
    ];
    for (let i of langs) {
        yield <option key={i.short} value={i.short}>{i.long}</option>
    }
}

const onChangeLang = (lang, i18n) => {
    i18n.changeLanguage(lang);
}

const LangPicker = (_) => {
    const { i18n } = useTranslation();

    return (
        <div className="is-right">
            <div className="select">
                <select onChange={e => onChangeLang(e.target.value, i18n)} value={i18n.language}>
                    {[...languages()]}
                </select>
            </div>
        </div>
    );
}

export default LangPicker;