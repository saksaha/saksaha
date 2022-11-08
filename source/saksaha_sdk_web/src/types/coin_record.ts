export interface CoinRecord {
  addr_pk: String,
  addr_sk: String,
  rho: String,
  r: String,
  s: String,
  v: String,
  cm: String,
  coin_status: CoinStatus,
  cm_idx: String,
  coin_idx: String,
  tx_hash: String,
};

export enum CoinStatus {
  Unconfirmed = "Unconfirmed ",
  Unused = "Unused",
  Used = "Used",
  Failed = "Failed",
};

