import {
  createSignal, createEffect
} from 'solid-js';

import type { Component, } from 'solid-js';
import { CoinManager } from 'saksaha';
import ActionPane from "../actionPane/ActionPane";

import * as styles from './Landing.css';
import ResultPane from '../resultPane/ResultPane';
import { CoinRecord, CoinStatus } from 'saksaha/src/types/coin';

const LandingPage: Component = () => {

  const [walletAddress, setWalletAddress] = createSignal("");

  const [coinManager, setCoinManager] = createSignal(new CoinManager());

  const [mrsSlots, setMrsSlots] = createSignal([""]);

  const [selectedCoin, setSelectedCoin] = createSignal<CoinRecord>({
    addr_pk: "",
    addr_sk: "",
    r: "",
    s: "",
    v: "",
    rho: "",
    cm: "",
    cm_idx: "",
    tx_hash: "",
    coin_idx: "",
    coin_status: CoinStatus.Failed
  });

  const [selectedMrsSlot, setSelectedMrsSlot] = createSignal("");

  createEffect(() => {
    console.log(walletAddress());
    console.log(coinManager());
    console.log("Selected Coin: " + selectedCoin().r);
  });

  return (
    <div class={styles.wrapper}>
      <div class={styles.left_pane}>
        <ActionPane
          coin_manager_setter={setCoinManager}
          wallet_addr_setter={setWalletAddress}
          mrs_slots_setter={setMrsSlots}
        />
      </div>
      <div class={styles.right_pane}>
        <ResultPane
          coin_manager={coinManager}
          wallet_addr={walletAddress}
          selected_coin={selectedCoin}
          selected_coin_setter={setSelectedCoin}
          selected_mrs_slot={selectedMrsSlot}
          selected_mrs_slot_setter={setSelectedMrsSlot}
        />
      </div >
    </div >

  );
};

export default LandingPage;
