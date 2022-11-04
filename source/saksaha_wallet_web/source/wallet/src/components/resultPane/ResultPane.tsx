import { CoinManager } from "saksaha";
import { CoinRecord } from "saksaha/src/types/coin";
import { Accessor, Component, createEffect, For, Setter } from "solid-js";
import * as styles from './ResultPane.css';

const ResultPane: Component<ResultPaneProps> = (props) => {
  const render_coin_manager = () => {
    return (
      <>
        <h3> coins</h3>
        {
          console.log("coin_manager: ", props.coin_manager.toString())
        }
        <For each={props.coin_manager().coins}>{(coin, i) =>
          <div
            class={
              props.selected_coin().cm == coin.cm ?
                styles.selected_coin_record : styles.coin_record
            }
            onClick={
              async () => { props.selected_coin_setter(coin) }
            }>
            coin [{i()}]: <br />
            {/* - addr_pk: {String(coin.addr_pk)} <br /> */}
            {/* - addr_sk: {String(coin.addr_sk)} <br /> */}
            {/* - rho: {String(coin.rho)} <br /> */}
            {/* - r: {String(coin.r)} <br /> */}
            {/* - s: {String(coin.s)} <br /> */}
            - value: [ {String(coin.v)} ] <br />
            - cm: {String(coin.cm)} <br />
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

  createEffect(() => {
    console.log("[#ResultPane] coin manager", props.coin_manager());

  });

  return (
    <>
      <h2>Saksaha Wallet</h2>
      <h3>7297b903877a957748b74068d63d6d566148197524099fc1df5cd9e8814c66c7</h3>
      <h3>wallet addr: {props.wallet_addr()}</h3>
      <div class={styles.result}>
        <div class={styles.result_coin_pane}>
          {props.wallet_addr() == "" ? null : render_coin_manager()}
        </div>
        <div class={styles.vertical_line}></div>
        <div class={styles.result_mrs_slot_pane}>
          {props.wallet_addr() == "" ? null : render_mrs_slots()}
        </div>
      </div>

    </>
  );
};

export default ResultPane;

interface ResultPaneProps {
  wallet_addr: Accessor<string>,
  coin_manager: Accessor<CoinManager>,
  selected_coin: Accessor<CoinRecord>,
  selected_coin_setter: Setter<CoinRecord>,
  selected_mrs_slot: Accessor<string>,
  selected_mrs_slot_setter: Setter<string>
}
