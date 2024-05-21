import i18n from 'i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import { initReactI18next } from 'react-i18next';
import translationDE from './i18n/de.json';
import translationEN from './i18n/en.json';
import translationRU from './i18n/ru.json';

const resources = {
  en: { translation: translationEN },
  de: { translation: translationDE },
  ru: { translation: translationRU },
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: 'en',
    detection: {
      order: ['navigator'],
      caches: []
    }
  });

export default i18n;
