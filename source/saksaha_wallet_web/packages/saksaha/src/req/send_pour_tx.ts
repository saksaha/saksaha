import Saksaha from "../saksaha";
import { Cm } from "../types/coin";

export const send_pour_tx = async (new_coin_data: SendPourTxParam) => {
  let axios: Saksaha = new Saksaha();

  const method = "send_pour_tx";

  const params = new_coin_data;

  const res = await axios.query(method, params);

  return res;
};

export const get_dummy_send_pour_tx_param = (): SendPourTxParam => {
  return {
    created_at: "created_at",
    data: [11],
    author_sig: "wallet_web_1",
    ctr_addr: 'Ok("null")',
    pi: [
      55, 55, 55, 55, 55, 55, 55, 55,
      55, 55, 55, 55, 55, 55, 55, 55,
      55, 55, 55, 55, 55, 55, 55, 55,
      55, 55, 55, 55, 55, 55, 55, 55,
    ],
    sns: [
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
    merkle_rts: [
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
  }
}

export interface SendPourTxParam {
  created_at: String,
  data: number[],
  author_sig: String,
  ctr_addr: String,
  pi: number[],
  sns: number[][],
  cms: number[][],
  merkle_rts: number[][]
}
