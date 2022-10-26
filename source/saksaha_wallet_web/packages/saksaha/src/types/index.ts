import { CoinManager, DEV_LOCAL_1_SK } from "./coin";

export interface Block {
  validator_sig: string;
  tx_hashes: string[];
  witness_sigs: string[];
  created_at: string;
  block_height: number;
  merkle_rt: number[];
  block_cm_count: number;
  block_hash: string;
};

export interface JsonResponse<T> {
  jsonrpc: string;
  error: Object;
  result: T;
  id: string;
};


export {
  CoinManager,
  DEV_LOCAL_1_SK
};
