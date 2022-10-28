import './style.css'
import { setupCounter } from './counter'
import * as Comlink from 'comlink';



const wasm = import('../../pkg');

// import init, { initThreadPool /* ... */ } from '../../pkg/sak_proof_wasm.js';
// await init();
// await initThreadPool(navigator.hardwareConcurrency);

(async function init() {
  // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
  let handlers = (await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ) as any)["handlers"];

  // '../../pkg/sak_proof_wasm.js'




  // function setupBtn(id: any) {
  //   // Handlers are named in the same way as buttons.
  //   let handler = handlers[id];
  //   // If handler doesn't exist, it's not supported.
  //   if (!handler) return;
  //   // Assign onclick handler + enable the button.
  //   Object.assign(document.getElementById(id), {
  //     async onclick() {
  //       let { rawImageData, time } = await handler({
  //         width,
  //         height,
  //         maxIterations
  //       });
  //       timeOutput.value = `${time.toFixed(2)} ms`;
  //       const imgData = new ImageData(rawImageData, width, height);
  //       ctx.putImageData(imgData, 0, 0);
  //     },
  //     disabled: false
  //   });
  // }

  // setupBtn('singleThread');
  // if (await handlers.supportsThreads) {
  //   setupBtn('multiThread');
  // }
})();






wasm.then(m => {
  console.log(1, Date.now());
  const arr_i32 = new Int32Array([21, 31]);

  m.greet(arr_i32);
  console.log(2, Date.now());
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
