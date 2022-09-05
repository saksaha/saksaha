use crate::LedgerError;
use crate::{cfs, LedgerDB};
use sak_crypto::{Bls12, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_proofs::CoinProof;
use sak_proofs::{Hasher, Proof};
use sak_types::{
    Cm, CmIdx, MintTx, MintTxCandidate, PourTx, PourTxCandidate, Tx, TxCtrOp,
    TxHash, TxType,
};

impl LedgerDB {
    pub(crate) async fn get_txs(
        &self,
        tx_hashes: &Vec<String>,
    ) -> Result<Vec<Tx>, LedgerError> {
        let mut ret = vec![];

        for tx_hash in tx_hashes {
            match self.get_tx(tx_hash).await? {
                Some(b) => ret.push(b),
                None => (),
            }
        }

        Ok(ret)
    }

    pub(crate) async fn get_tx(
        &self,
        tx_hash: &String,
    ) -> Result<Option<Tx>, LedgerError> {
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

    fn get_mint_tx(&self, tx_hash: &String) -> Result<Tx, LedgerError> {
        let created_at = self
            .get_tx_created_at(tx_hash)?
            .ok_or("created_at does not exist")?;

        let data = self.get_data(tx_hash)?.ok_or("data does not exist")?;

        let author_sig = self
            .get_author_sig(tx_hash)?
            .ok_or("author_sig does not exist")?;

        let ctr_addr = self.get_ctr_addr(tx_hash)?;

        let cm_count = self.get_cm_count(tx_hash)?.ok_or("cms should exist")?;

        let cms = self.get_cms(tx_hash)?.ok_or("cms should exist")?;

        let v = self.get_v(tx_hash)?.ok_or("v should exist")?;

        let k = self.get_k(tx_hash)?.ok_or("k should exist")?;

        let s = self.get_s(tx_hash)?.ok_or("s shoudl exist")?;

        let mut cm_idxes = vec![];
        for cm in &cms {
            let cm_idx = self
                .get_cm_idx_by_cm(cm)?
                .ok_or("cm_idx_1 does not exist")?;

            cm_idxes.push(cm_idx);
        }

        let tx_candidate = MintTxCandidate::new(
            created_at, data, author_sig, ctr_addr, cms, v, k, s,
        );

        let tx = Tx::Mint(MintTx::new(tx_candidate, cm_idxes));

        Ok(tx)
    }

    fn get_pour_tx(
        &self,
        // db: &DB,
        // schema: &LedgerDBSchema,
        tx_hash: &String,
    ) -> Result<Tx, LedgerError> {
        let created_at = self
            .get_tx_created_at(tx_hash)?
            .ok_or("created_at does not exist")?;

        let data = self.get_data(tx_hash)?.ok_or("data does not exist")?;

        let author_sig = self
            .get_author_sig(tx_hash)?
            .ok_or("author_sig does not exist")?;

        let ctr_addr = self.get_ctr_addr(tx_hash)?;

        let pi = self.get_pi(tx_hash)?.ok_or("pi should exist")?;

        let sns = self.get_sns(tx_hash)?.ok_or("sn_1 should exist")?;

        let cms = self.get_cms(tx_hash)?.ok_or("cms should exist")?;

        let merkle_rt = self
            .get_prf_merkle_rt(tx_hash)?
            .ok_or("merkle_root should exist")?;

        let mut cm_idxes = vec![];
        for cm in &cms {
            let cm_idx = self
                .get_cm_idx_by_cm(&cm)?
                .ok_or("cm_idx_1 does not exist")?;

            cm_idxes.push(cm_idx);
        }

        let tx_candidate = PourTxCandidate::new(
            created_at, data, author_sig, ctr_addr, pi, sns, cms, merkle_rt,
        );

        let tx = Tx::Pour(PourTx::new(tx_candidate, cm_idxes));

        Ok(tx)
    }
}

impl LedgerDB {
    pub(crate) fn batch_put_tx(
        &self,
        batch: &mut WriteBatch,
        tx: &Tx,
        // cm_idx_count: &mut u128,
    ) -> Result<TxHash, LedgerError> {
        let tx_hash = match tx {
            Tx::Mint(t) => self.batch_put_mint_tx(
                batch, t,
                // cm_idx_count
            ),
            Tx::Pour(t) => self.batch_put_pour_tx(
                batch, t,
                // cm_idx_count
            ),
        }?;

        // println!("cm_idx_count :{:?}", cm_idx_count);

        Ok(tx_hash)
    }

    pub(crate) fn batch_put_mint_tx(
        &self,
        batch: &mut WriteBatch,
        tx: &MintTx,
        // cm_idx_count: &mut u128,
    ) -> Result<TxHash, LedgerError> {
        let tc = &tx.tx_candidate;

        let tx_hash = tc.get_tx_hash();

        // let cm_idx = self.batch_increment_cm_idx(batch, &tc.cm)?;

        self.batch_put_tx_type(batch, tx_hash, tc.get_tx_type())?;

        self.batch_put_cms(batch, tx_hash, &tc.cms)?;

        for (cm, cm_idx) in std::iter::zip(&tc.cms, &tx.cm_idxes) {
            self.batch_put_cm_cm_idx(batch, cm, cm_idx)?;
            self.batch_put_cm_idx_cm(batch, cm_idx, cm)?;
        }

        self.batch_put_cm_count(batch, tx_hash, &tc.cm_count)?;

        self.batch_put_tx_created_at(batch, tx_hash, &tc.created_at)?;

        self.batch_put_data(batch, tx_hash, &tc.data)?;

        self.batch_put_author_sig(batch, tx_hash, &tc.author_sig)?;

        self.batch_put_ctr_addr(batch, tx_hash, &tc.ctr_addr)?;

        self.batch_put_v(batch, tx_hash, &tc.v)?;

        self.batch_put_k(batch, tx_hash, &tc.k)?;

        self.batch_put_s(batch, tx_hash, &tc.s)?;

        // self.batch_put_tx_height(batch, tx_hash, &tx.tx_height)?;

        // self.batch_put_tx_hash_by_height(batch, &tx.tx_height, tx_hash)?;

        let tx_ctr_op = tc.get_ctr_op();

        match tx_ctr_op {
            TxCtrOp::ContractDeploy => {
                self.batch_put_tx_hash_by_contract_addr(
                    batch,
                    &tc.ctr_addr,
                    tx_hash,
                )?;
            }
            TxCtrOp::ContractCall => {}
            TxCtrOp::None => {}
        }

        Ok(tx_hash.clone())
    }

    pub(crate) fn batch_put_pour_tx(
        &self,
        batch: &mut WriteBatch,
        tx: &PourTx,
        // cm_idx_count: &mut u128,
    ) -> Result<TxHash, LedgerError> {
        let tc = &tx.tx_candidate;

        let tx_hash = tc.get_tx_hash();

        println!("tx_hash in put: {}", tx_hash);

        self.batch_put_tx_hash_by_sn(batch, &tc.sns, tx_hash)?;

        self.batch_put_tx_type(batch, tx_hash, tc.get_tx_type())?;

        self.batch_put_tx_created_at(batch, tx_hash, &tc.created_at)?;

        self.batch_put_data(batch, tx_hash, &tc.data)?;

        self.batch_put_author_sig(batch, tx_hash, &tc.author_sig)?;

        self.batch_put_ctr_addr(batch, tx_hash, &tc.ctr_addr)?;

        // self.batch_put_tx_height(batch, tx_hash, &tx.tx_height)?;

        // self.batch_put_tx_hash_by_height(batch, &tx.tx_height, tx_hash)?;

        self.batch_put_pi(batch, tx_hash, &tc.pi)?;

        self.batch_put_sns(batch, tx_hash, &tc.sns)?;

        // self.batch_put_sn_2(batch, tx_hash, &tc.sn_2)?;

        self.batch_put_cms(batch, tx_hash, &tc.cms)?;
        // self.batch_put_cm_1(batch, tx_hash, &tc.cm_1)?;

        // self.batch_put_cm_2(batch, tx_hash, &tc.cm_2)?;

        // self.batch_put_cm_cm_idx(batch, &tc.cm_1, cm_idx_count)?;

        // self.batch_put_cm_cm_idx(batch, &tc.cm_2, &(*cm_idx_count + 1))?;

        for (cm, cm_idx) in std::iter::zip(&tc.cms, &tx.cm_idxes) {
            self.batch_put_cm_cm_idx(batch, cm, cm_idx)?;
            self.batch_put_cm_idx_cm(batch, cm_idx, cm)?;
        }

        self.batch_put_cm_count(batch, tx_hash, &tc.cm_count)?;

        // self.batch_put_cm_cm_idx(batch, &tc.cm_1, &tx.cm_idx_1)?;
        // self.batch_put_cm_idx_cm(batch, &tx.cm_idx_1, &tc.cm_1)?;

        // self.batch_put_cm_cm_idx(batch, &tc.cm_2, &tx.cm_idx_2)?;
        // self.batch_put_cm_idx_cm(batch, &tx.cm_idx_2, &tc.cm_2)?;

        self.batch_put_prf_merkle_rt(batch, tx_hash, &tc.merkle_rt)?;

        let tx_ctr_op = tc.get_ctr_op();

        match tx_ctr_op {
            TxCtrOp::ContractDeploy => {
                self.batch_put_tx_hash_by_contract_addr(
                    batch,
                    &tc.ctr_addr,
                    tx_hash,
                )?;
            }
            TxCtrOp::ContractCall => {}
            TxCtrOp::None => {}
        }

        Ok(tx_hash.clone())
    }
}
