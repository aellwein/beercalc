import React, { Suspense, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import NavBar from '../navbar/NavBar';
import Footer from './footer/Footer';

const App = () => {
  const { t } = useTranslation();
  useEffect(() => {
    document.title = t('title');
  });

  return (
    <Suspense fallback='loading'>
      <NavBar />
      <Footer />
    </Suspense >
  );
}

export default App;