import Saksaha from "../saksaha";
import { Cm } from "../types/coin";

export const send_mint_tx = async (new_coin_data: SendMintTxParam) => {
  let axios: Saksaha = new Saksaha();

  const method = "send_mint_tx";

  const params = new_coin_data;

  const res = await axios.query(method, params);

  return res;
};

export const get_dummy_send_mint_tx_param = (): SendMintTxParam => {
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

export interface SendMintTxParam {
  created_at: String,
  data: number[],
  author_sig: String,
  ctr_addr: String,
  cms: Cm[],
  v: number[],
  k: number[],
  s: number[],
}
