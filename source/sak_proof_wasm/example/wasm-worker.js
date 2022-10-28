
import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

// Wrap wasm-bindgen exports (the `generate` function) to add time measurement.
function wrapExports({ generate }) {
    return ({ width, height, maxIterations }) => {
        const start = performance.now();
        const rawImageData = generate(width, height, maxIterations);
        const time = performance.now() - start;
        return {
            // Little perf boost to transfer data to the main thread w/o copying.
            rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
            time
        };
    };
}

async function initHandlers() {
    console.log(5)
    let [multiThread] = await Promise.all([

        (async () => {
            console.log(6)
            // If threads are unsupported in this browser, skip this handler.
            if (!(await threads())) return;
            const multiThread = await import(
                '../pkg/sak_proof_wasm.js'
            );
            console.log(7)
            await multiThread.default();
            await multiThread.initThreadPool(navigator.hardwareConcurrency);
            return wrapExports(multiThread);
        })()
    ]);

    return Comlink.proxy({
        // singleThread,
        supportsThreads: !!multiThread,
        multiThread
    });
}

Comlink.expose({
    handlers: initHandlers()
});
