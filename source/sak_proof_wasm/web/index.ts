import * as Comlink from "comlink";
import { WasmHandler } from "./wasm-worker";

console.log(555);

let worker = new Worker(new URL("./wasm-worker.ts", import.meta.url), {
  type: "module",
});

const get_proof = async (p: number[]) => {
  let a = await Comlink.wrap<Comlink.Remote<WasmHandler>>(worker).handlers;

  let { proof, time } = await a.multiThread(p);

  return { proof, time };
};

declare global {
  interface Window {
    saksaha: any;
  }
}

window.saksaha = {};
window.saksaha.get_proof = get_proof;
