import { CoinManager } from 'saksaha';
import { Component, createSignal, Setter, Accessor } from 'solid-js';
import * as styles from './ActionPane.css';
import FaucetBtn from './faucetBtn/FaucetBtn';
import ProofGenBtn from './proofGenBtn/ProofGenBtn';

interface ActionPaneProps {
  coin_manager_setter: Setter<CoinManager>,//
  coin_manager: Accessor<CoinManager>,
  wallet_addr_setter: Setter<string>,//
  mrs_slots_setter: Setter<string[]>
}

const ActionPane: Component<ActionPaneProps> = (props) => {
  let coin_manager_setter: Setter<CoinManager> = props.coin_manager_setter;
  let coin_manager: Accessor<CoinManager> = props.coin_manager;
  let wallet_addr_setter: Setter<string> = props.wallet_addr_setter;
  // let mrs_slots_setter: Setter<string[]> = props.mrs_slots_setter;

  const [walletAddrInput, setWalletAddrInput] = createSignal("");
  const [loginStatus, setLoginStatus] = createSignal(false);


  const handle_log_in = (wallet_id: string) => {
    let res = new CoinManager(wallet_id);

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
            <ProofGenBtn />
            <br />
            <FaucetBtn
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
