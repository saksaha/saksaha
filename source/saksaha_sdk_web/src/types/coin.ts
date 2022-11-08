import { comm, mimc } from "../crypto";

export interface Coin {
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

export type Cm = number[];

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

const get_zero_u8_32 = (): number[] => {
  let zero_u8_32: number[] = [];

  for (let i = 0; i < 32; i++) {

    zero_u8_32[i] = 0x00;

  }

  return zero_u8_32;
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

const gen_random_u8_32 = (): number[] => {
  let b: number[] = [];

  for (let i = 0; i < 32; i++) {
    let rand_byte = Math.floor(Math.random() * 255);

    b.push(rand_byte);
  };

  return b;
}

export const get_new_coin_data = (value: number): Coin => {
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

export const get_dummy_new_coin_data = (): Coin => {
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
