import './style.css'
import { setupCounter } from './counter'

const wasm = import('../../pkg');

// import { a } from 'saksaha';

// console.log(11, a());

wasm.then(m => {
  console.log(1, m);
  m.greet('power');
});


document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    123
    <div class="card">
      <button id="counter" type="button"></button>
    </div>
  </div>
`

setupCounter(document.querySelector<HTMLButtonElement>('#counter')!)
