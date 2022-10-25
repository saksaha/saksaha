import { Component } from 'solid-js';
import * as styles from './ActionPane.css';

const ActionPane: Component = (props) => {

  let a = props;

  return (
    <div class={styles.input_row}>
      whoami
      {/* <input */}
      {/*   class={styles.input_single_field} */}
      {/*   placeholder={ */}
      {/*     loginStatus() ? */}
      {/*       "  ID: " + walletAddressRecord().slice(0, 8) + "..." */}
      {/*       : styles.InputWalletString */}
      {/*   } */}
      {/*   onChange={(e) => setWalletAddress(e.currentTarget.value)} */}
      {/*   value={walletAddress()} */}
      {/*   disabled={ */}
      {/*     loginStatus() ? true : false */}
      {/*   } */}
      {/* /> */}

      {/* <div */}
      {/*   class={styles.input_btn} */}
      {/*   onClick={() => { */}
      {/*     let wallet_address = walletAddress(); */}

      {/*     handle_log_in(wallet_address); */}
      {/*   }} */}
      {/* > */}
      {/*   SEND */}
      {/* </div> */}
    </div>
    // {
    //   loginStatus() ?
    //     <div class={styles.input_row}>
    //       <input
    //         class={styles.input_two_field}
    //         placeholder={styles.SlotIdString}
    //         onChange={(e) => setSlotId(e.currentTarget.value)}></input>

    //       <input
    //         class={styles.input_two_field}
    //         placeholder={styles.SlotDataString}
    //         onChange={(e) => setData(e.currentTarget.value)}></input>

    //       <div
    //         class={styles.input_btn}
    //         onClick={() => {
    //           console.log(slotId() + data());
    //         }}

    //       >
    //         SEND
    //       </div>
    //     </div>

    //     : null
    // }

  );

};

export default ActionPane;
