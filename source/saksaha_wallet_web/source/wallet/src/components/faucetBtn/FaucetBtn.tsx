import { Saksaha } from "saksaha";
import { Component } from "solid-js";
import * as styles from './FaucetBtn.css';

const get_tx = async (tx_hash: String) => {
  console.log("get tx");
  let axios: Saksaha = new Saksaha();

  const method = "get_tx";
  const params = {
    hash: tx_hash
  };

  let res = await axios.query(method, params);

  if (res === undefined) {
    return false;
  } else {
    return true;

  }
}

const send_mint_tx = async (new_coin_data: any) => {
  console.log("send mint tx");

  let axios: Saksaha = new Saksaha();

  const method = "send_mint_tx";
  const params = new_coin_data;

  const res = await axios.query(method, params);


  return res;
};

const gen_random_u8_32 = () => {
  let b: number[] = [];
  for (let i = 0; i < 32; i++) {
    let rand_byte = Math.floor(Math.random() * 255);

    b.push(rand_byte);
  };

  return b;
}

const get_new_coin_data = () => {

  return {

    // created_at: String
    created_at: "created_at",

    //   data: Vec<u8>,
    data: [11, 22, 33],

    //   author_sig: String,
    author_sig: "wallet_web_1",

    //   ctr_addr: Option<String>,
    ctr_addr: 'wallet_web_ctr_addr',

    // cms: Vec < [u8; 32]>,
    cms: [[
      11, 11, 11, 11, 11, 11, 11, 11,
      22, 22, 22, 22, 22, 22, 22, 22,
      33, 33, 33, 33, 33, 33, 33, 33,
      44, 44, 44, 44, 44, 44, 44, 44,
    ], [
      11, 11, 11, 11, 11, 11, 11, 11,
      22, 22, 22, 22, 22, 22, 22, 22,
      33, 33, 33, 33, 33, 33, 33, 33,
      44, 44, 44, 44, 44, 44, 44, 44,
    ]],

    // cms: [cm_1, cm_2],

    //   v: [u8; 32],
    v: [
      11, 11, 11, 11, 11, 11, 11, 11,
      22, 22, 22, 22, 22, 22, 22, 22,
      33, 33, 33, 33, 33, 33, 33, 33,
      44, 44, 44, 44, 44, 44, 44, 44,
    ],

    // k: [u8; 32],
    k: [
      11, 11, 11, 11, 11, 11, 11, 11,
      22, 22, 22, 22, 22, 22, 22, 22,
      33, 33, 33, 33, 33, 33, 33, 33,
      44, 44, 44, 44, 44, 44, 44, 44,
    ],

    // // s: [u8; 32],
    // s: [
    //   11, 11, 11, 11, 11, 11, 11, 11,
    //   22, 22, 22, 22, 22, 22, 22, 22,
    //   33, 33, 33, 33, 33, 33, 33, 33,
    //   44, 44, 44, 44, 44, 44, 44, 44,
    // ]
    s: gen_random_u8_32(),
  };
}



const FaucetBtn: Component = () => {


  return (
    <>
      <input type="button" class={styles.faucet_btn} value="Faucet Button" onclick={
        async () => {
          const new_coin_data = get_new_coin_data();

          let tx_hash = await send_mint_tx(new_coin_data);

          while (!await get_tx(tx_hash)) {
            // wait until the tx be in a new block...
          }

          console.log(new_coin_data)
        }
      } />
    </>
  );
};

export default FaucetBtn;
