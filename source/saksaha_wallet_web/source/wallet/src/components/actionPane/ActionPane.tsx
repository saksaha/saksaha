import { CoinManager, Saksaha, DEV_LOCAL_1_SK } from 'saksaha';
import { Component, createSignal, Setter, createEffect } from 'solid-js';
import * as styles from './ActionPane.css';


const ActionPane: Component = (props: { coin_manager_setter: Setter<CoinManager>, wallet_addr_setter: Setter<string>, mrs_slots_setter: Setter<string[]> }) => {
  let coin_manager_setter: Setter<CoinManager> = props.coin_manager_setter;

  let wallet_addr_setter: Setter<string> = props.wallet_addr_setter;

  let mrs_slots_setter: Setter<string[]> = props.mrs_slots_setter;

  const [walletAddrInput, setWalletAddrInput] = createSignal("");

  const [loginStatus, setLoginStatus] = createSignal(false);


  const handle_log_in = (wallet_id: string) => {
    // let res = new CoinManager(DEV_LOCAL_1_SK);
    let res = new CoinManager(wallet_id);

    coin_manager_setter(res);

    wallet_addr_setter(wallet_id);

    setWalletAddrInput("");

    setLoginStatus(true);

  };

  createEffect(() => {
    if (loginStatus() == true) {
      console.log("send request to dev_local_1");
      // TODO: send request `get my mrs slots`
      // const saksaha = new Saksaha(["http://localhost:34418/rpc/v0"]);
      // saksaha.query("get_mrs_slot_list", { DEV_LOCAL_1_SK }).then((res) => {
      //   console.log(55, res);
      // });
    }
  });




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
