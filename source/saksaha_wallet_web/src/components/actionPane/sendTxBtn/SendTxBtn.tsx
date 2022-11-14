import { SendPourTxParam, send_pour_tx } from "saksaha";
import { Component } from "solid-js";
import * as Comlink from "comlink";
import { Coin, get_dummy_new_coin_data } from "saksaha";

import * as styles from "./SendTxBtn.css";
import { WasmHandler } from "../../../wasm-worker";

let worker = new Worker(new URL("../../../wasm-worker.ts", import.meta.url), {
  type: "module",
});

const get_proof = async (p: number[]) => {
  let a = await Comlink.wrap<Comlink.Remote<WasmHandler>>(worker).handlers;

  let { proof, time } = await a.multiThread(p);

  return { proof, time };
};

const SendTxBtn: Component = () => {
  return (
    <>
      <input
        type="button"
        class={styles.send_tx_btn}
        value="Send Tx Btn"
        onclick={async () => {
          console.log("send tx btn clicked");

          const dummy_new_coin: Coin = get_dummy_new_coin_data();

          const int_arr = [11, 22];
          const dummy_u8_32 = [
            55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55,
            55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55, 55,
          ];

          console.log("[+] gen proof starts....wait plz");

          let { proof, time } = await get_proof(int_arr);
          var proof_array = Array.prototype.slice.call(proof);

          console.log("[+] pi type", typeof proof_array, proof_array);
          console.log("[+] dummy type", typeof dummy_u8_32, dummy_u8_32);

          const dummy_send_pour_tx_data: SendPourTxParam = {
            created_at: dummy_new_coin.created_at,
            data: [11],
            author_sig: "wallet_web_1",
            ctr_addr: 'Ok("null")',
            pi: proof_array,
            sns: [dummy_u8_32, dummy_u8_32],
            cms: [dummy_u8_32, dummy_u8_32],
            merkle_rts: [dummy_u8_32, dummy_u8_32],
          };

          console.log("[+] gen proof end!");

          console.log("proof: ", proof_array);

          let tx_hash = await send_pour_tx(dummy_send_pour_tx_data);

          console.log("time:  ", time, "ms");

          console.log("send tx finished");
        }}
      />
    </>
  );
};

export default SendTxBtn;
