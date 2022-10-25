import {
  createSignal, createEffect, For
} from 'solid-js';

import type { Component, } from 'solid-js';
import { Saksaha, CoinManager } from 'saksaha';
import ActionPane from "../actionPane/ActionPane";

import * as styles from './Landing.css';

const LandingPage: Component = () => {

  const [loginStatus, setLoginStatus] = createSignal(false);

  const [walletAddress, setWalletAddress] = createSignal("");

  const [walletAddressRecord, setWalletAddressRecord] = createSignal("");

  const [slotId, setSlotId] = createSignal("");

  const [data, setData] = createSignal("");


  const [coinManager, setCoinManager] = createSignal(new CoinManager())

  // createEffect(() => {
  //   const saksaha = new Saksaha(["http://localhost:34418/rpc/v0"]);
  //   // saksaha.query("get_block_list", {}).then((res) => {
  //   // console.log(55, res.block_list);

  //   // });
  // });

  const handle_log_in = (wallet_id: String) => {
    // let res = new CoinManager("7297b903877a957748b74068d63d6d566148197524099fc1df5cd9e8814c66c7");
    let res = new CoinManager(wallet_id);

    setCoinManager(res);

    console.log(coinManager());

    setWalletAddressRecord(walletAddress());

    setWalletAddress("");

    setLoginStatus(true);
  };

  createEffect(() => {
    console.log(walletAddress());
  });

  return (
    <div class={styles.wrapper}>

      <div class={styles.left_pane}>
        <div class={styles.input_row}>
          <input
            class={styles.input_single_field}
            placeholder={
              loginStatus() ?
                "  ID: " + walletAddressRecord().slice(0, 8) + "..."
                : styles.InputWalletString
            }
            onChange={(e) => setWalletAddress(e.currentTarget.value)}
            value={walletAddress()}
            disabled={
              loginStatus() ? true : false
            }
          />

          <div
            class={styles.input_btn}
            onClick={() => {
              let wallet_address = walletAddress();

              handle_log_in(wallet_address);
            }}
          >
            SEND
          </div>
        </div>
        {
          loginStatus() ?
            <div class={styles.input_row}>
              <input
                class={styles.input_two_field}
                placeholder={styles.SlotIdString}
                onChange={(e) => setSlotId(e.currentTarget.value)}></input>

              <input
                class={styles.input_two_field}
                placeholder={styles.SlotDataString}
                onChange={(e) => setData(e.currentTarget.value)}></input>

              <div
                class={styles.input_btn}
                onClick={() => {
                  console.log(slotId() + data());
                }}

              >
                SEND
              </div>
            </div>

            : null}
      </div>

      <div class={styles.right_pane}>
        <h2>Saksaha Wallet</h2>

        <div class={styles.result}>
          <h2> Result </h2>
          <div class={styles.result_body}>
            <For each={coinManager().coins}>{(coin, i) =>
              <div class={styles.coin_record}>
                coin [{i()}]: <br />
                - addr_pk: {String(coin.addr_pk)} <br />
                - addr_sk: {String(coin.addr_sk)} <br />
                - rho: {String(coin.rho)} <br />
                - r: {String(coin.r)} <br />
                - s: {String(coin.s)} <br />
                - value: {String(coin.v)} <br />
                - cm: {String(coin.cm)} <br />
                - status: {String(coin.coin_status)} <br />
                - cm index: {String(coin.cm_idx)} <br />
                - transaction hash: {String(coin.tx_hash)} <br />
              </div>
            }
            </For>
          </div>
        </div>
      </div >
    </div >

  );
};

export default LandingPage;
