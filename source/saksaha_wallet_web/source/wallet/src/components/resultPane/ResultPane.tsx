import { CoinManager } from "saksaha";
import { CoinRecord } from "saksaha/src/types/coin";
import { Accessor, Component, createEffect, createSignal, For, Setter } from "solid-js";
import * as styles from './ResultPane.css';

const ResultPane: Component = (props: {
  wallet_addr: Accessor<string>,
  coin_manager: Accessor<CoinManager>,
  selected_coin: Accessor<CoinRecord>,
  selected_coin_setter: Setter<CoinRecord>,
  selected_mrs_slot: Accessor<string>,
  selected_mrs_slot_setter: Setter<string>
}) => {

  let wallet_addr: Accessor<string> = props.wallet_addr;

  let coin_manager: Accessor<CoinManager> = props.coin_manager;

  let selected_coin: Accessor<CoinRecord> = props.selected_coin;

  let selected_coin_setter: Setter<CoinRecord> = props.selected_coin_setter;

  let selected_mrs_slot: Accessor<string> = props.selected_mrs_slot;

  let selected_mrs_slot_setter: Setter<string> = props.selected_mrs_slot_setter;

  const render_coin_manager = () => {
    return (
      <>
        <h3> coins</h3>
        <For each={coin_manager().coins}>{(coin, i) =>
          <div
            class={
              selected_coin().cm == coin.cm ?
                styles.selected_coin_record : styles.coin_record
            }
            onClick={
              () => { selected_coin_setter(coin) }
            }>
            coin [{i()}]: <br />
            {/* - addr_pk: {String(coin.addr_pk)} <br /> */}
            {/* - addr_sk: {String(coin.addr_sk)} <br /> */}
            {/* - rho: {String(coin.rho)} <br /> */}
            {/* - r: {String(coin.r)} <br /> */}
            {/* - s: {String(coin.s)} <br /> */}
            - value: [ {String(coin.v)} ] <br />
            {/* - cm: {String(coin.cm)} <br /> */}
            - status: [ {String(coin.coin_status)} ] <br />
            {/* - cm index: {String(coin.cm_idx)} <br /> */}
            - transaction hash: [ {String(coin.tx_hash)} ] <br />
          </div>
        }
        </For >
      </>
    );
  }

  const render_mrs_slots = () => {
    return (
      <>
        <h3>MRS Slots</h3>
        <div class={styles.mrs_slot}>slot 1</div>
      </>
    );
  }

  return (
    <>

      <h2>Saksaha Wallet</h2>
      <h3>wallet addr: {wallet_addr()}</h3>
      <div class={styles.result}>
        <div class={styles.result_coin_pane}>
          {wallet_addr() == "" ? null : render_coin_manager()}
        </div>
        <div class={styles.vertical_line}></div>
        <div class={styles.result_mrs_slot_pane}>
          {wallet_addr() == "" ? null : render_mrs_slots()}
        </div>
      </div>

    </>
  );

};

export default ResultPane;
