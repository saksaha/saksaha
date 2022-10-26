import { CoinManager } from "saksaha";
import { Accessor, Component, createEffect, For } from "solid-js";
import * as styles from './ResultPane.css';

const ResultPane: Component = (props: { wallet_addr: Accessor<string>, coin_manager: Accessor<CoinManager> }) => {
  let wallet_addr: Accessor<string> = props.wallet_addr;

  let coin_manager: Accessor<CoinManager> = props.coin_manager;



  createEffect(() => {
    console.log("[!] Result Pane, wallet_addr:  " + wallet_addr());
    console.log("[!] Result pane, coin_manager: " + coin_manager().coins);
  });


  const render_coin_manager = () => {
    return (
      <div class={styles.result_body}>
        <For each={coin_manager().coins}>{(coin, i) =>
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
    );
  }

  return (
    <div class={styles.asdf}>

      <h2>Saksaha Wallet</h2>
      <h3>wallet addr: {wallet_addr}</h3>
      <div class={styles.result}>
        {wallet_addr() == "" ? null : render_coin_manager()}
      </div>

    </div>
  );

};

export default ResultPane;
