import { CoinManager } from "./coin_manager";

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
};
