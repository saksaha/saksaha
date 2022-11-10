import { createSignal, createEffect } from "solid-js";
import type { Component } from "solid-js";

import * as styles from "./Landing.css";

import ActionPane from "../actionPane/ActionPane";
import ResultPane from "../resultPane/ResultPane";

import { CoinManager } from "saksaha";
// import { CoinRecord, CoinStatus } from "saksaha";

const LandingPage: Component = () => {
  // const [walletAddress, setWalletAddress] = createSignal("");
  // const [coinManager, setCoinManager] = createSignal(new CoinManager());
  // const [selectedCoin, setSelectedCoin] = createSignal<CoinRecord>({
  //   addr_pk: "",
  //   addr_sk: "",
  //   r: "",
  //   s: "",
  //   v: "",
  //   rho: "",
  //   cm: "",
  //   cm_idx: "",
  //   tx_hash: "",
  //   coin_idx: "",
  //   coin_status: CoinStatus.Failed,
  // });
  // const [mrsSlots, setMrsSlots] = createSignal([""]);
  // const [selectedMrsSlot, setSelectedMrsSlot] = createSignal("");

  // createEffect(() => {
  //   console.log("[#Landing] coin manager:", coinManager());
  //   console.log("Selected Coin: " + selectedCoin().cm);
  // });

  return (
    <div class={styles.wrapper}>
      {/* <div class={styles.left_pane}> */}
      {/*   <ActionPane */}
      {/*     coin_manager_setter={setCoinManager} */}
      {/*     coin_manager={coinManager} */}
      {/*     wallet_addr={walletAddress} */}
      {/*     wallet_addr_setter={setWalletAddress} */}
      {/*     mrs_slots_setter={setMrsSlots} */}
      {/*   /> */}
      {/* </div> */}
      {/* <div class={styles.right_pane}> */}
      {/*   <ResultPane */}
      {/*     coin_manager={coinManager} */}
      {/*     wallet_addr={walletAddress} */}
      {/*     selected_coin={selectedCoin} */}
      {/*     selected_coin_setter={setSelectedCoin} */}
      {/*     selected_mrs_slot={selectedMrsSlot} */}
      {/*     selected_mrs_slot_setter={setSelectedMrsSlot} */}
      {/*   /> */}
      {/* </div> */}
    </div>
  );
};

export default LandingPage;
