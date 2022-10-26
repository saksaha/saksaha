use crate::{cfs, LedgerDB, PourTxEntity};
use crate::{MachineError, MintTxEntity};
use sak_kv_db::{Direction, IteratorMode, WriteBatch};
use sak_types::{MintTx, MintTxCandidate, PourTx, PourTxCandidate, Tx, TxCtrOp, TxHash, TxType};

impl LedgerDB {
    pub async fn get_txs(&self, tx_hashes: &Vec<String>) -> Result<Vec<Tx>, MachineError> {
        let mut ret = vec![];

        for tx_hash in tx_hashes {
            match self.get_tx(tx_hash).await? {
                Some(b) => ret.push(b),
                None => (),
            }
        }

        Ok(ret)
    }

    pub async fn get_tx(&self, tx_hash: &String) -> Result<Option<Tx>, MachineError> {
        let tx_type = self
            .get_tx_type(tx_hash)?
            .ok_or(format!("Tx type does not exist, tx_hash: {}", tx_hash))?;

        let tx = match tx_type {
            TxType::Mint => self.get_mint_tx(tx_hash),
            TxType::Pour => self.get_pour_tx(tx_hash),
            _ => Err(format!("Invalid tx type, {:?}", tx_type).into()),
        }?;

        Ok(Some(tx))
    }

    fn get_mint_tx(&self, tx_hash: &String) -> Result<Tx, MachineError> {
        let mint_tx_entity = self
            .get_raw_mint_tx_entity(tx_hash)?
            .ok_or("MintTxEntity should exist")?;

        let tx_candidate = MintTxCandidate::new(
            mint_tx_entity.created_at,
            mint_tx_entity.data,
            mint_tx_entity.author_sig,
            Some(mint_tx_entity.ctr_addr),
            mint_tx_entity.cms,
            mint_tx_entity.v,
            mint_tx_entity.k,
            mint_tx_entity.s,
        );

        let tx = Tx::Mint(MintTx::new(tx_candidate, mint_tx_entity.cm_idxes));

        Ok(tx)
    }

    fn get_pour_tx(&self, tx_hash: &String) -> Result<Tx, MachineError> {
        let pour_tx_entity = self
            .get_raw_pour_tx_entity(tx_hash)?
            .ok_or("PourTxEntity should exist")?;

        let tx_candidate = PourTxCandidate::new(
            pour_tx_entity.created_at,
            pour_tx_entity.data,
            pour_tx_entity.author_sig,
            Some(pour_tx_entity.ctr_addr),
            pour_tx_entity.pi,
            pour_tx_entity.sns,
            pour_tx_entity.cms,
            pour_tx_entity.prf_merkle_rts,
        );

        let tx = Tx::Pour(PourTx::new(tx_candidate, pour_tx_entity.cm_idxes));

        Ok(tx)
    }
}

impl LedgerDB {
    pub fn batch_put_tx(&self, batch: &mut WriteBatch, tx: &Tx) -> Result<TxHash, MachineError> {
        println!("\n>> tx to put: {}", tx);

        let tx_hash = match tx {
            Tx::Mint(t) => {
                let tc = &t.tx_candidate;

                let mint_tx_entity = MintTxEntity {
                    tx_hash: tc.get_tx_hash().to_string(),
                    tx_type: tc.get_tx_type(),
                    cms: tc.get_cms().to_vec(),
                    cm_idxes: tx.get_cm_idxes().to_vec(),
                    cm_count: tc.cm_count,
                    created_at: tc.created_at.to_owned(),
                    data: tc.data.to_owned(),
                    author_sig: tc.author_sig.to_owned(),
                    ctr_addr: tc.ctr_addr.to_owned(),
                    v: tc.v,
                    k: tc.k,
                    s: tc.s,
                    tx_ctr_op: tc.get_ctr_op(),
                };

                self.batch_put_mint_tx(batch, mint_tx_entity)
            }
            Tx::Pour(t) => {
                let tc = &t.tx_candidate;

                let pour_tx_entity = PourTxEntity {
                    tx_hash: tc.get_tx_hash().to_string(),
                    tx_type: tc.get_tx_type(),
                    cms: tc.get_cms().to_vec(),
                    cm_idxes: tx.get_cm_idxes().to_vec(),
                    cm_count: tc.cm_count,
                    created_at: tc.created_at.to_owned(),
                    data: tc.data.to_owned(),
                    author_sig: tc.author_sig.to_owned(),
                    ctr_addr: tc.ctr_addr.to_owned(),
                    pi: tc.pi,
                    sns: tc.sns,
                    prf_merkle_rts: tc.merkle_rts,
                    tx_ctr_op: tc.get_ctr_op(),
                };

                self.batch_put_pour_tx(batch, pour_tx_entity)
            }
        }?;

        Ok(tx_hash)
    }

    pub fn batch_put_mint_tx(
        &self,
        batch: &mut WriteBatch,
        tx_entity: MintTxEntity,
    ) -> Result<TxHash, MachineError> {
        let tx_hash = &tx_entity.tx_hash;

        self.batch_put_tx_type(batch, tx_hash, tx_entity.tx_type)?;
        for (cm, cm_idx) in std::iter::zip(&tx_entity.cms, &tx_entity.cm_idxes) {
            self.batch_put_cm_cm_idx(batch, cm, cm_idx)?;
            self.batch_put_cm_idx_cm(batch, cm_idx, cm)?;
        }
        self.batch_put_mint_tx_entity(batch, tx_hash, &tx_entity)?;

        match tx_entity.tx_ctr_op {
            TxCtrOp::ContractDeploy => {
                self.batch_put_tx_hash_by_contract_addr(batch, &tx_entity.ctr_addr, tx_hash)?;
            }
            TxCtrOp::ContractCall => {}
            TxCtrOp::None => {}
        }

        Ok(tx_hash.clone())
    }

    pub fn batch_put_pour_tx(
        &self,
        batch: &mut WriteBatch,
        tx_entity: PourTxEntity,
    ) -> Result<TxHash, MachineError> {
        let tx_hash = &tx_entity.tx_hash;

        self.batch_put_tx_type(batch, tx_hash, tx_entity.tx_type)?;
        for (cm, cm_idx) in std::iter::zip(&tx_entity.cms, &tx_entity.cm_idxes) {
            self.batch_put_cm_cm_idx(batch, cm, cm_idx)?;
            self.batch_put_cm_idx_cm(batch, cm_idx, cm)?;
        }
        for (idx, sn) in tx_entity.sns.iter().enumerate() {
            let key = format!("{}_{}", tx_hash, idx);
            self.batch_put_tx_hash_by_sn(batch, &sn, tx_hash)?;
        }

        self.batch_put_pour_tx_entity(batch, tx_hash, &tx_entity)?;

        match tx_entity.tx_ctr_op {
            TxCtrOp::ContractDeploy => {
                self.batch_put_tx_hash_by_contract_addr(batch, &tx_entity.ctr_addr, tx_hash)?;
            }
            TxCtrOp::ContractCall => {}
            TxCtrOp::None => {}
        }

        Ok(tx_hash.clone())
    }

    // fn get_cms_iteratively(&self, tx_hash: &TxHash) -> Result<Vec<[u8; 32]>, MachineError> {
    //     let tx_hash_bytes = tx_hash.as_bytes();
    //     let mut v = vec![];

    //     let mut cm_iter = {
    //         let cf = self.make_cf_handle(&self.db, cfs::CM)?;
    //         self.db
    //             .iterator_cf(&cf, IteratorMode::From(tx_hash_bytes, Direction::Forward))
    //     };

    //     loop {
    //         let (key, cm) = if let Some(v) = cm_iter.next() {
    //             v
    //         } else {
    //             break;
    //         };

    //         if key.starts_with(tx_hash_bytes) {
    //             let mut arr: [u8; 32] = Default::default();
    //             arr.clone_from_slice(&cm);

    //             v.push(arr);
    //         } else {
    //             break;
    //         }
    //     }

    //     if v.len() < 1 {
    //         return Err(format!("At least one cm should exist, tx_hash: {}", tx_hash).into());
    //     }

    //     Ok(v)
    // }

    // fn get_sns_iteratively(&self, tx_hash: &TxHash) -> Result<Vec<[u8; 32]>, MachineError> {
    //     let tx_hash_bytes = tx_hash.as_bytes();

    //     let mut v = vec![];

    //     let mut sn_iter = {
    //         let cf = self.make_cf_handle(&self.db, cfs::SN)?;
    //         self.db
    //             .iterator_cf(&cf, IteratorMode::From(tx_hash_bytes, Direction::Forward))
    //     };

    //     loop {
    //         let (key, sn) = if let Some(v) = sn_iter.next() {
    //             v
    //         } else {
    //             break;
    //         };

    //         if key.starts_with(tx_hash_bytes) {
    //             let mut arr: [u8; 32] = Default::default();
    //             arr.clone_from_slice(&sn);

    //             v.push(arr);
    //         } else {
    //             break;
    //         }
    //     }

    //     if v.len() < 1 {
    //         return Err(format!("At least one sn should exist, tx_hash: {}", tx_hash).into());
    //     }

    //     Ok(v)
    // }

    // fn get_merkle_rts_iteratively(&self, tx_hash: &TxHash) -> Result<Vec<[u8; 32]>, MachineError> {
    //     let tx_hash_bytes = tx_hash.as_bytes();
    //     let mut v = vec![];

    //     let mut merkle_rt_iter = {
    //         let cf = self.make_cf_handle(&self.db, cfs::PRF_MERKLE_RT)?;
    //         self.db
    //             .iterator_cf(&cf, IteratorMode::From(tx_hash_bytes, Direction::Forward))
    //     };

    //     loop {
    //         let (key, merkle_rt) = if let Some(v) = merkle_rt_iter.next() {
    //             v
    //         } else {
    //             break;
    //         };

    //         if key.starts_with(tx_hash_bytes) {
    //             let mut arr: [u8; 32] = Default::default();
    //             arr.clone_from_slice(&merkle_rt);

    //             v.push(arr);
    //         } else {
    //             break;
    //         }
    //     }

    //     if v.len() < 1 {
    //         return Err(
    //             format!("At least one merkle_rt should exist, tx_hash: {}", tx_hash).into(),
    //         );
    //     }

    //     Ok(v)
    // }
}
