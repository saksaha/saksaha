import { CoinManager } from 'saksaha';
import { Component, createSignal, Setter } from 'solid-js';
import * as styles from './ActionPane.css';

const ActionPane: Component = (props: { coin_manager_setter: Setter<CoinManager>, wallet_addr_setter: Setter<string> }) => {


  const [walletAddrInput, setWalletAddrInput] = createSignal("");

  const [loginStatus, setLoginStatus] = createSignal(false);


  const handle_log_in = (wallet_id: string) => {
    // let res = new CoinManager("7297b903877a957748b74068d63d6d566148197524099fc1df5cd9e8814c66c7");
    let res = new CoinManager(wallet_id);

    props.coin_manager_setter(res);

    props.wallet_addr_setter(wallet_id);


    // setWalletAddressRecord(walletAddress());

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
              "  ID: " + walletAddrInput().slice(0, 8) + "..."
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
            // placeholder={styles.SlotIdString}
            // onChange={(e) => setSlotId(e.currentTarget.value)}
            />
            <div
              class={styles.input_btn}
              onClick={() => {
                // console.log(slotId() + data());
              }}
            >
              SEND
            </div>
          </div >

          : null
      }
    </>

  );

};

export default ActionPane;
