import { CoinManager } from '../../../../../../saksaha_sdk_web/src';
import { Component, createSignal, Setter, Accessor } from 'solid-js';
import * as styles from './ActionPane.css';
import FaucetBtn from './faucetBtn/FaucetBtn';
import SendTxBtn from './sendTxBtn/SendTxBtn';

interface ActionPaneProps {
  coin_manager_setter: Setter<CoinManager>,//
  coin_manager: Accessor<CoinManager>,
  wallet_addr: Accessor<string>,
  wallet_addr_setter: Setter<string>,//
  mrs_slots_setter: Setter<string[]>
}

const ActionPane: Component<ActionPaneProps> = (props) => {
  let coin_manager_setter: Setter<CoinManager> = props.coin_manager_setter;
  let coin_manager: Accessor<CoinManager> = props.coin_manager;
  let wallet_addr: Accessor<string> = props.wallet_addr;
  let wallet_addr_setter: Setter<string> = props.wallet_addr_setter;
  // let mrs_slots_setter: Setter<string[]> = props.mrs_slots_setter;

  const [walletAddrInput, setWalletAddrInput] = createSignal("");
  const [loginStatus, setLoginStatus] = createSignal(false);


  const handle_log_in = (wallet_id: string) => {
    // localStorage
    const key = wallet_id + ":" + "coin_manager";

    let res: any;

    if (!localStorage.getItem(key)) {
      console.log("new id");

      res = new CoinManager(wallet_id);

      localStorage.setItem(key, JSON.stringify(res));

    } else {
      console.log("existed id");

      let tmp_res = localStorage.getItem(key) as string;

      res = JSON.parse(tmp_res);
    }

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
          placeholder={styles.InputWalletString}
          onChange={(e) => setWalletAddrInput(e.currentTarget.value)}
          disabled={
            loginStatus() ? true : false
          }
        />

        {
          loginStatus() ?
            null :
            <div
              class={styles.input_btn}
              onClick={() => {
                let wallet_address = walletAddrInput();

                handle_log_in(wallet_address);
              }}
            >SEND</div>
        }
      </div >
      {
        loginStatus() ?
          <div class={styles.input_row}>
            <input
              class={styles.input_single_field}
              placeholder={styles.InputData}
            />
            <div class={styles.input_btn}>SEND</div>
          </div>
          : null
      }
      {
        loginStatus() ?
          <>
            <SendTxBtn />
            <br />
            <FaucetBtn
              wallet_addr={wallet_addr}
              coin_manager={coin_manager}
              coin_manager_setter={coin_manager_setter}
            />
          </>
          : null
      }
    </>
  );
};

export default ActionPane;
