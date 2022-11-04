import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

// Wrap wasm-bindgen exports (the `greet` function) to add time measurement.
function wrapExports({ greet }: { greet: any }) {
  return async (int_arr: number[]) => {
    // let bb: number[] = [44, 55];

    const start = performance.now();

    const greet_result = greet(int_arr);

    const time = performance.now() - start;

    return {
      // Little perf boost to transfer data to the main thread w/o copying.
      greet_result: Comlink.transfer(greet_result, [greet_result.buffer]),
      time
    };
  };
}

async function initHandlers() {
  let [multiThread] = await Promise.all([
    (async () => {

      if (!(await threads())) return;

      const multiThread = await import(
        '../../../pkg/sak_proof_wasm.js'
      );

      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);

      return wrapExports(multiThread);
    })()
  ]);

  return Comlink.proxy({
    multiThread
  });
}

export interface WasmHandler {
  handlers: Promise<{
    multiThread: (
      int_arr: number[]
    ) => {
      greet_result: any;
      time: number;
    }
  }>
};

Comlink.expose({
  handlers: await initHandlers()
});
