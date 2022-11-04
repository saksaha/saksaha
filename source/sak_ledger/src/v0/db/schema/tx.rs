use crate::{LedgerCols, LedgerDB, PourTxEntity};
use crate::{LedgerError, MintTxEntity};
use sak_kv_db::WriteBatch;
use sak_types::{MintTx, MintTxCandidate, PourTx, PourTxCandidate, Tx, TxCtrOp, TxHash, TxType};

impl LedgerDB {
    pub async fn get_txs(&self, tx_hashes: &Vec<String>) -> Result<Vec<Tx>, LedgerError> {
        let mut ret = vec![];

        for tx_hash in tx_hashes {
            if let Some(b) = self.get_tx(tx_hash).await? {
                ret.push(b);
            }
        }

        Ok(ret)
    }

    pub async fn get_tx(&self, tx_hash: &String) -> Result<Option<Tx>, LedgerError> {
        let tx_type = self
            .get(LedgerCols::TxType, tx_hash.as_bytes())?
            .ok_or(format!("Tx type does not exist, tx_hash: {}", tx_hash))?;

        let tx = match tx_type {
            TxType::Mint => self.get_mint_tx(tx_hash),
            TxType::Pour => self.get_pour_tx(tx_hash),
            _ => Err(format!("Invalid tx type, {:?}", tx_type).into()),
        }?;

        Ok(Some(tx))
    }

    fn get_mint_tx(&self, tx_hash: &String) -> Result<Tx, LedgerError> {
        let mint_tx_entity: MintTxEntity = self
            .get(LedgerCols::MintTxEntity, tx_hash.as_bytes())?
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

    fn get_pour_tx(&self, tx_hash: &String) -> Result<Tx, LedgerError> {
        let pour_tx_entity: PourTxEntity = self
            .get(LedgerCols::PourTxEntity, tx_hash.as_bytes())?
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
    pub fn batch_put_tx(&self, batch: &mut WriteBatch, tx: &Tx) -> Result<TxHash, LedgerError> {
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
                    pi: tc.pi.to_vec(),
                    sns: tc.sns.to_vec(),
                    prf_merkle_rts: tc.merkle_rts.to_vec(),
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
    ) -> Result<TxHash, LedgerError> {
        let tx_hash = &tx_entity.tx_hash;

        self.put(
            batch,
            LedgerCols::MintTxEntity,
            tx_hash.as_bytes(),
            &tx_entity,
        )?;

        self.put(
            batch,
            LedgerCols::TxType,
            tx_hash.as_bytes(),
            &tx_entity.tx_type,
        )?;

        self.put(batch, LedgerCols::Data, tx_hash.as_bytes(), &tx_entity.data)?;

        for (cm, cm_idx) in std::iter::zip(&tx_entity.cms, &tx_entity.cm_idxes) {
            self.put(batch, LedgerCols::CMIdxByCM, cm, cm_idx)?;
            self.put(batch, LedgerCols::CMByCMIdx, &cm_idx.to_be_bytes(), cm)?;
        }

        match tx_entity.tx_ctr_op {
            TxCtrOp::ContractDeploy => {
                self.put(
                    batch,
                    LedgerCols::TxHashByCtrAddr,
                    tx_entity.ctr_addr.as_bytes(),
                    tx_hash,
                )?;
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
    ) -> Result<TxHash, LedgerError> {
        let tx_hash = &tx_entity.tx_hash;

        self.put(
            batch,
            LedgerCols::PourTxEntity,
            tx_hash.as_bytes(),
            &tx_entity,
        )?;

        self.put(
            batch,
            LedgerCols::TxType,
            tx_hash.as_bytes(),
            &tx_entity.tx_type,
        )?;

        self.put(batch, LedgerCols::Data, tx_hash.as_bytes(), &tx_entity.data)?;

        for (cm, cm_idx) in std::iter::zip(&tx_entity.cms, &tx_entity.cm_idxes) {
            self.put(batch, LedgerCols::CMIdxByCM, cm, cm_idx)?;
            self.put(batch, LedgerCols::CMByCMIdx, &cm_idx.to_be_bytes(), cm)?;
        }
        for (idx, sn) in tx_entity.sns.iter().enumerate() {
            let _key = format!("{}_{}", tx_hash, idx);

            self.put(batch, LedgerCols::TxHashBySN, sn, tx_hash)?;
        }

        match tx_entity.tx_ctr_op {
            TxCtrOp::ContractDeploy => {
                self.put(
                    batch,
                    LedgerCols::TxHashByCtrAddr,
                    tx_entity.ctr_addr.as_bytes(),
                    tx_hash,
                )?;
            }
            TxCtrOp::ContractCall => {}
            TxCtrOp::None => {}
        }

        Ok(tx_hash.clone())
    }
}
