import { threads } from "wasm-feature-detect";
import * as Comlink from "comlink";

import wasm from "../../prebuild/sak_proof_wasm/sak_proof_wasm.js";

// Wrap wasm-bindgen exports (the `greet` function) to add time measurement.
function wrapExports({ greet }: { greet: any }) {
  return async (int_arr: number[]) => {
    // let bb: number[] = [44, 55];

    const start = performance.now();

    const proof = greet(int_arr);

    const time = performance.now() - start;

    return {
      // Little perf boost to transfer data to the main thread w/o copying.
      proof: Comlink.transfer(proof, [proof.buffer]),
      time,
    };
  };
}

async function initHandlers() {
  let [multiThread] = await Promise.all([
    (async () => {
      if (!(await threads())) return;

      // const multiThread = await import(
      //   "../../prebuild/sak_proof_wasm/sak_proof_wasm.js"
      // );
      //
      const multiThread = await wasm();

      // await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);

      return wrapExports(multiThread);
    })(),
  ]);

  return Comlink.proxy({
    multiThread,
  });
}

export interface WasmHandler {
  handlers: Promise<{
    multiThread: (int_arr: number[]) => {
      proof: any;
      time: number;
    };
  }>;
}

Comlink.expose({
  handlers: initHandlers(),
});
