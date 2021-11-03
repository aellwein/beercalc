import { Suspense, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import NavBar from '../navbar/NavBar';
import Footer from '../footer/Footer';

const App = () => {
  const { t } = useTranslation();

  useEffect(() => {
    document.title = t('title');
  });

  return (
    <Suspense fallback='loading'>
        <div className="container mx-auto p-3 dark:bg-gray-800">
          <NavBar />
          <Footer />
      </div>
    </Suspense >
  );
}

export default App;