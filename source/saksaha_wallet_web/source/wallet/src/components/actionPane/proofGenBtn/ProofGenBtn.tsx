import { Component } from "solid-js";
import * as styles from './ProofGenBtn.css';
import * as Comlink from 'comlink';
import { WasmHandler } from "./wasm-worker";

let worker = new Worker(new URL('./wasm-worker.ts', import.meta.url), {
  type: 'module'
});


const get_proof = async (p: number[]) => {
  let a =
    await Comlink.wrap<Comlink.Remote<WasmHandler>>(worker).handlers;

  let { greet_result, time } = await a.multiThread(p);

  return { greet_result, time };
};

const ProofGenBtn: Component = () => {
  return (
    <>
      {/* <input type="button" id="multiThread" value="Gen Proof" disabled /> */}
      <input type="button" class={styles.proof_gen_btn} value="Proof Gen Btn" onclick={
        async () => {
          console.log('gen proof btn clicked');

          const int_arr = [11, 22];

          console.log("[+] gen proof starts....wait plz");

          let { greet_result, time } = await get_proof(int_arr);

          console.log("[+] gen proof end!");

          console.log("proof: ", greet_result);

          console.log("time:  ", time, 'ms');
        }
      } />
    </>
  );
};

export default ProofGenBtn;
