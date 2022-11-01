import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

// Wrap wasm-bindgen exports (the `greet` function) to add time measurement.
function wrapExports({ greet }) {
  return ({ int_arr }) => {
    console.log(9995)
    // const start = performance.now();
    const start = performance.now();
    const greet_result = greet(int_arr);
    const time = performance.now() - start;

    console.log(9996, greet_result);
    // const time = performance.now() - start;
    return {
      // Little perf boost to transfer data to the main thread w/o copying.
      // rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
      greet_result: Comlink.transfer(greet_result, [greet_result.buffer]),
      time
    };
  };
}

async function initHandlers() {
  let [multiThread] = await Promise.all([
    (async () => {
      console.log(666)
      // If threads are unsupported in this browser, skip this handler.
      console.log("threads ok: ", await threads());

      if (!(await threads())) return;

      const multiThread = await import(
        '../pkg/sak_proof_wasm.js'
      );

      console.log(777)

      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);

      return wrapExports(multiThread);
    })()
  ]);

  return Comlink.proxy({
    multiThread
  });
}

Comlink.expose({
  handlers: initHandlers()
});
