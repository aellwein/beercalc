import React, { Suspense } from 'react';
import AlcoholCalc from '../alcohol-calc/AlcoholCalc';

const App = () => {
  return (
    <Suspense fallback='loading'>
      <AlcoholCalc />
    </Suspense>
  );
}

export default App;