import { CoinManager, Saksaha } from "saksaha";
import { comm, mimc } from "saksaha/src/crypto";
import { CoinRecord, CoinStatus } from "saksaha/src/types/coin";
import { Accessor, Component, createEffect, Setter } from "solid-js";
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

const send_mint_tx = async (new_coin_data: SendMintTxParam) => {
  console.log("send mint tx");

  let axios: Saksaha = new Saksaha();

  const method = "send_mint_tx";
  const params = new_coin_data;

  const res = await axios.query(method, params);


  return res;
};

const gen_random_u8_32 = (): number[] => {
  let b: number[] = [];

  for (let i = 0; i < 32; i++) {
    let rand_byte = Math.floor(Math.random() * 255);

    b.push(rand_byte);
  };

  return b;
}

const get_created_at = (): String => {
  let today = new Date();

  let year = today.getFullYear();
  let month = ('0' + (today.getMonth() + 1)).slice(-2);
  let day = ('0' + today.getDate()).slice(-2);
  let dateString = year + '_' + month + '_' + day;

  let hours = ('0' + today.getHours()).slice(-2);
  let minutes = ('0' + today.getMinutes()).slice(-2);
  let seconds = ('0' + today.getSeconds()).slice(-2);
  let timeString = hours + ':' + minutes + ':' + seconds;

  return dateString + "-" + timeString;
}

const convert_value_into_le_u8_32 = (value: number): number[] => {
  let value_u8_32: number[] = [];

  for (let i = 0; i < 24; i++) {
    value_u8_32[i] = 0x00;
  }

  for (let i = 24; i < 32; i++) {
    value_u8_32[i] = (value % 256);
    value = Math.floor(value / 256);
  }

  return value_u8_32;
}
const get_zero_u8_32 = (): number[] => {
  let zero_u8_32: number[] = [];
  for (let i = 0; i < 32; i++) {
    zero_u8_32[i] = 0x00;
  }

  return zero_u8_32;
}

const get_new_coin_data = (value: number): Coin => {
  const created_at = get_created_at();
  const addr_sk = gen_random_u8_32();
  const addr_pk = mimc(addr_sk, get_zero_u8_32());
  const rho = gen_random_u8_32();
  const r = gen_random_u8_32();
  const s = gen_random_u8_32();
  const v = convert_value_into_le_u8_32(value);
  const k = comm(r, addr_pk, rho);
  const cm = comm(s, v, k);

  const new_coin: Coin = {
    created_at,
    addr_sk,
    addr_pk,
    rho,
    r,
    s,
    v,
    k,
    cm
  };

  return new_coin;
}

const get_dummy_new_coin_data = (): Coin => {
  const created_at = get_created_at();

  const addr_sk = [
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
  ];

  const addr_pk = mimc(addr_sk, get_zero_u8_32());

  const rho = [
    22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
    22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
  ];

  const r = [
    33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33,
    33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33,
  ];

  const s = [
    44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44,
    44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44,
  ];

  const v = convert_value_into_le_u8_32(100);

  const k = comm(r, addr_pk, rho);

  const cm = comm(s, v, k);

  const new_coin: Coin = {
    created_at,
    addr_sk,
    addr_pk,
    rho,
    r,
    s,
    v,
    k,
    cm
  };

  return new_coin;
}

const get_dummy_send_mint_tx_param = (): SendMintTxParam => {
  return {
    created_at: "created_at",
    data: [11],
    author_sig: "wallet_web_1",
    ctr_addr: 'Ok("null")',
    cms: [
      // gen_random_u8_32(),
      [
        11, 11, 11, 11, 11, 11, 11, 11,
        11, 11, 11, 11, 11, 11, 11, 11,
        11, 11, 11, 11, 11, 11, 11, 11,
        11, 11, 11, 11, 11, 11, 11, 11,
      ],
      [
        22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22,
        22, 22, 22, 22, 22, 22, 22, 22,
      ],
    ],
    v:
      [
        33, 33, 33, 33, 33, 33, 33, 33,
        33, 33, 33, 33, 33, 33, 33, 33,
        33, 33, 33, 33, 33, 33, 33, 33,
        33, 33, 33, 33, 33, 33, 33, 33,
      ],
    k: [
      44, 44, 44, 44, 44, 44, 44, 44,
      44, 44, 44, 44, 44, 44, 44, 44,
      44, 44, 44, 44, 44, 44, 44, 44,
      44, 44, 44, 44, 44, 44, 44, 44,
    ],
    s: [
      55, 55, 55, 55, 55, 55, 55, 55,
      55, 55, 55, 55, 55, 55, 55, 55,
      55, 55, 55, 55, 55, 55, 55, 55,
      55, 55, 55, 55, 55, 55, 55, 55,
    ],

  }
}

const encode_hex = (arr: number[]): String => {
  let res: String = "";

  for (let i = 0; i < arr.length; i++) {
    let hex = arr[i].toString(16).padStart(2, '0');

    res += hex;
  }

  return res;
}


const click_fn = async (
  props: FBParam
) => {
  {
    const dummy_new_coin: Coin = get_dummy_new_coin_data();
    // const dummy_new_coin: Coin = get_new_coin_data(100);

    const dummy_send_mint_tx_data: SendMintTxParam = {
      created_at: dummy_new_coin.created_at,
      data: [11],
      author_sig: "wallet_web_1",
      ctr_addr: 'Ok("null")',
      cms: [
        dummy_new_coin.cm,
      ],
      v: dummy_new_coin.v,
      k: dummy_new_coin.k,
      s: dummy_new_coin.s,
    }

    let tx_hash = await send_mint_tx(dummy_send_mint_tx_data);
    console.log("tx_hash: ", tx_hash);

    alert("Your coin is mining... (~5 secs)")
    while (!await get_tx(tx_hash)) {
      // wait until the tx be in a new block...
    }

    const coin_record: CoinRecord = {
      addr_pk: encode_hex(dummy_new_coin.addr_pk),
      addr_sk: encode_hex(dummy_new_coin.addr_sk),
      rho: encode_hex(dummy_new_coin.rho),
      r: encode_hex(dummy_new_coin.r),
      s: encode_hex(dummy_new_coin.s),
      v: encode_hex(dummy_new_coin.v),
      cm: encode_hex(dummy_new_coin.cm),
      coin_status: CoinStatus.Unused,
      cm_idx: "10",
      coin_idx: props.coin_manager().coins.length.toString(),
      tx_hash: tx_hash,
    };

    let aaaa: CoinManager = new CoinManager();

    let origin_coin_manager = props.coin_manager();

    for (let i = 0; i < origin_coin_manager.coins.length; i++) {
      aaaa.coins.push(origin_coin_manager.coins[i]);
    }

    aaaa.coins.push(coin_record);

    props.coin_manager_setter(aaaa);

    console.log(props.coin_manager().coins.length);
  }
}


const FaucetBtn: Component<FBParam> = (props) => {

  createEffect(() => {
    console.log("[#FaucetBtn] coin manager:", props.coin_manager());
  })

  return (
    <>
      <input type="button" class={styles.faucet_btn} value="Faucet Button" onclick={
        async () => {
          click_fn(props)
        }
      } />
    </>
  );
};

export default FaucetBtn;

interface Coin {
  created_at: String,
  addr_sk: number[],
  addr_pk: number[],
  rho: number[],
  r: number[],
  s: number[],
  v: number[],
  k: number[],
  cm: Cm,
};

type Cm = number[];

interface SendMintTxParam {
  created_at: String,
  data: number[],
  author_sig: String,
  ctr_addr: String,
  cms: Cm[],
  v: number[],
  k: number[],
  s: number[],
}

interface FBParam {
  coin_manager: Accessor<CoinManager>,
  coin_manager_setter: Setter<CoinManager>,//
}
