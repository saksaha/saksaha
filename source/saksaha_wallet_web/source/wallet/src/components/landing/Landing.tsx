import {
  createSignal, createEffect, For
} from 'solid-js';

import type { Component, } from 'solid-js';
import { Saksaha, CoinManager } from 'saksaha';
import ActionPane from "../actionPane/ActionPane";

import * as styles from './Landing.css';
import ResultPane from '../resultPane/ResultPane';

const LandingPage: Component = () => {

  const [walletAddress, setWalletAddress] = createSignal("");

  const [coinManager, setCoinManager] = createSignal(new CoinManager())

  // createEffect(() => {
  //   const saksaha = new Saksaha(["http://localhost:34418/rpc/v0"]);
  //   // saksaha.query("get_block_list", {}).then((res) => {
  //   // console.log(55, res.block_list);

  //   // });
  // });

  createEffect(() => {
    console.log(coinManager());
  });


  return (
    <div class={styles.wrapper}>
      <div class={styles.left_pane}>
        <ActionPane coin_manager_setter={setCoinManager} wallet_addr_setter={setWalletAddress} />
      </div>
      <div class={styles.right_pane}>
        <ResultPane coin_manager={coinManager} wallet_addr={walletAddress} />
      </div >
    </div >

  );
};

export default LandingPage;
