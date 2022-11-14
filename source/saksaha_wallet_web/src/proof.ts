import * as Comlink from "comlink";
import { WasmHandler } from "./wasm-worker";

let worker = new Worker(new URL("../src/wasm-worker.ts", import.meta.url), {
  type: "module",
});

const get_proof = async (p: number[]) => {
  let a = await Comlink.wrap<Comlink.Remote<WasmHandler>>(worker).handlers;

  let { proof, time } = await a.multiThread(p);

  return { proof, time };
};

export default get_proof;
