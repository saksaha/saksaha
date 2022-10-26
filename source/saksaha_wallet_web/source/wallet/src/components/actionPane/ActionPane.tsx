import { CoinManager } from 'saksaha';
import { Component, createSignal, Setter } from 'solid-js';
import * as styles from './ActionPane.css';

const ActionPane: Component = (props: { coin_manager_setter: Setter<CoinManager>, wallet_addr_setter: Setter<string> }) => {
  let coin_manager_setter: Setter<CoinManager> = props.coin_manager_setter;

  let wallet_addr_setter: Setter<string> = props.wallet_addr_setter;

  const [walletAddrInput, setWalletAddrInput] = createSignal("");

  const [loginStatus, setLoginStatus] = createSignal(false);


  const handle_log_in = (wallet_id: string) => {
    let res = new CoinManager("7297b903877a957748b74068d63d6d566148197524099fc1df5cd9e8814c66c7");
    // let res = new CoinManager(wallet_id);

    coin_manager_setter(res);

    wallet_addr_setter(wallet_id);

    setWalletAddrInput("");

    setLoginStatus(true);
  };


  return (
    <>
      <div class={styles.input_row}>
        <input
          class={styles.input_single_field}
          placeholder={
            loginStatus() ?
              "  Login Done"
              : styles.InputWalletString
          }
          onChange={(e) => setWalletAddrInput(e.currentTarget.value)}
          value={walletAddrInput()}
          disabled={
            loginStatus() ? true : false
          }
        />

        <div
          class={styles.input_btn}
          onClick={() => {
            let wallet_address = walletAddrInput();

            handle_log_in(wallet_address);
          }}
        >
          SEND
        </div>
      </div >

      {
        loginStatus() ?
          <div class={styles.input_row}>
            <input
              class={styles.input_single_field}
              placeholder={styles.InputData}
            />
            <div class={styles.input_btn}>SEND</div>
          </div >
          : null
      }
    </>

  );

};

export default ActionPane;
