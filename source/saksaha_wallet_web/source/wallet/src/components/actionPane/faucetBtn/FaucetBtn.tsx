import { CoinManager, get_tx, SendMintTxParam, send_mint_tx } from "saksaha";
import { Coin, get_dummy_new_coin_data } from "saksaha/src/types/coin";
import { CoinRecord, CoinStatus } from "saksaha/src/types/coin_record";
import { Accessor, Component, Setter } from "solid-js";
import * as styles from './FaucetBtn.css';


const encode_hex = (arr: number[]): String => {
  let res: String = "";

  for (let i = 0; i < arr.length; i++) {
    let hex = arr[i].toString(16).padStart(2, '0');

    res += hex;
  }

  return res;
}


const click_fn = async (
  props: FaucetBtnProps
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

    let new_coin_manager: CoinManager = new CoinManager();

    let origin_coin_manager = props.coin_manager();

    for (let i = 0; i < origin_coin_manager.coins.length; i++) {
      new_coin_manager.coins.push(origin_coin_manager.coins[i]);
    }

    new_coin_manager.coins.push(coin_record);

    props.coin_manager_setter(new_coin_manager);
  }
}


const FaucetBtn: Component<FaucetBtnProps> = (props) => {

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

interface FaucetBtnProps {
  coin_manager: Accessor<CoinManager>,
  coin_manager_setter: Setter<CoinManager>,//
}
