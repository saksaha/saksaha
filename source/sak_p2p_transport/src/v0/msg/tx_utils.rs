use crate::{utils, TrptError};
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};
use sak_types::{MintTx, MintTxCandidate, PourTx, PourTxCandidate, Tx};

pub(crate) fn parse_mint_tx_candidate(
    parse: &mut Parse,
) -> Result<MintTxCandidate, TrptError> {
    let data = {
        let p = parse.next_bytes()?;
        p.to_vec()
    };

    let created_at = {
        let k = parse.next_bytes()?;
        std::str::from_utf8(k.as_ref())?.into()
    };

    let author_sig = {
        let k = parse.next_bytes()?;
        std::str::from_utf8(k.as_ref())?.into()
    };

    let ctr_addr = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let cm_count = parse.next_int()?;

    let mut cms = Vec::with_capacity(cm_count as usize);

    for _ in 0..cm_count {
        let b = parse.next_bytes()?;

        cms.push(utils::convert_bytes_into_u8_32(b)?);
    }

    let v = {
        let p = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(p)?
    };

    let k = {
        let p = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(p)?
    };

    let s = {
        let p = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(p)?
    };

    let _tx_hash: String = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let mint_tx_candidate = MintTxCandidate::new(
        created_at,
        data,
        author_sig,
        Some(ctr_addr),
        cms,
        // cm_count,
        v,
        k,
        s,
    );

    Ok(mint_tx_candidate)
}

pub(crate) fn parse_mint_tx(parse: &mut Parse) -> Result<Tx, TrptError> {
    let mint_tx_candidate = parse_mint_tx_candidate(parse)?;

    let cm_count = parse.next_int()?;

    let mut cm_idxes = Vec::with_capacity(cm_count as usize);

    for _ in 0..cm_count {
        cm_idxes.push(parse.next_int()?);
    }

    let mint_tx = MintTx {
        tx_candidate: mint_tx_candidate,
        cm_idxes,
    };

    let tx = Tx::Mint(mint_tx);

    Ok(tx)
}

pub(crate) fn parse_pour_tx_candidate(
    parse: &mut Parse,
) -> Result<PourTxCandidate, TrptError> {
    let data = {
        let p = parse.next_bytes()?;
        p.to_vec()
    };

    let created_at = {
        let k = parse.next_bytes()?;
        std::str::from_utf8(k.as_ref())?.into()
    };

    let author_sig = {
        let k = parse.next_bytes()?;
        std::str::from_utf8(k.as_ref())?.into()
    };

    let ctr_addr = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let pi = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let sn_count = parse.next_int()?;

    let mut sns = Vec::with_capacity(sn_count as usize);

    for _ in 0..sn_count {
        let b = parse.next_bytes()?;

        sns.push(utils::convert_bytes_into_u8_32(b)?);
    }

    let cm_count = parse.next_int()?;

    let mut cms = Vec::with_capacity(cm_count as usize);

    for _ in 0..cm_count {
        let b = parse.next_bytes()?;

        cms.push(utils::convert_bytes_into_u8_32(b)?);
    }

    let merkle_rt_count = parse.next_int()?;

    let mut merkle_rts = Vec::with_capacity(merkle_rt_count as usize);

    for _ in 0..merkle_rt_count {
        let b = parse.next_bytes()?;

        merkle_rts.push(utils::convert_bytes_into_u8_32(b)?);
    }

    let _tx_hash: String = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let pour_tx = PourTxCandidate::new(
        created_at,
        data,
        author_sig,
        Some(ctr_addr),
        pi,
        sns,
        cms,
        merkle_rts,
    );

    Ok(pour_tx)
}

pub(crate) fn parse_pour_tx(parse: &mut Parse) -> Result<Tx, TrptError> {
    let pour_tx_candidate = parse_pour_tx_candidate(parse)?;

    let cm_count = parse.next_int()?;

    let mut cm_idxes = Vec::with_capacity(cm_count as usize);

    for _ in 0..cm_count {
        cm_idxes.push(parse.next_int()?);
    }

    let pour_tx = PourTx {
        tx_candidate: pour_tx_candidate,
        cm_idxes,
    };

    let tx = Tx::Pour(pour_tx);

    Ok(tx)
}

pub(crate) fn put_mint_tx_candidate_into_frame(
    frame: &mut Frame,
    tc: MintTxCandidate,
) {
    let tx_hash = tc.get_tx_hash().to_string();
    let tc_type = vec![tc.get_tx_type() as u8];

    frame.push_bulk(Bytes::copy_from_slice(&tc_type));
    frame.push_bulk(Bytes::from(tc.data));
    frame.push_bulk(Bytes::from(tc.created_at));
    frame.push_bulk(Bytes::from(tc.author_sig));
    frame.push_bulk(Bytes::from(tc.ctr_addr));
    frame.push_int(tc.cm_count);
    for cm in tc.cms.iter() {
        frame.push_bulk(Bytes::copy_from_slice(cm));
    }
    frame.push_bulk(Bytes::copy_from_slice(&tc.v));
    frame.push_bulk(Bytes::copy_from_slice(&tc.k));
    frame.push_bulk(Bytes::copy_from_slice(&tc.s));
    frame.push_bulk(Bytes::from(tx_hash));
}

pub(crate) fn put_mint_tx_into_frame(frame: &mut Frame, tx: MintTx) {
    let tc = tx.tx_candidate;

    put_mint_tx_candidate_into_frame(frame, tc);
}

pub(crate) fn put_pour_tx_candidate_into_frame(
    frame: &mut Frame,
    tc: PourTxCandidate,
) {
    let tx_hash = tc.get_tx_hash().to_string();
    let tc_type = vec![tc.get_tx_type() as u8];

    frame.push_bulk(Bytes::copy_from_slice(&tc_type));
    frame.push_bulk(Bytes::from(tc.data));
    frame.push_bulk(Bytes::from(tc.created_at));
    frame.push_bulk(Bytes::from(tc.author_sig));
    frame.push_bulk(Bytes::from(tc.ctr_addr));
    frame.push_bulk(Bytes::from(tc.pi));
    frame.push_int(tc.sn_count);

    for sn in tc.sns.iter() {
        frame.push_bulk(Bytes::copy_from_slice(sn));
    }

    frame.push_int(tc.cm_count);

    for cm in tc.cms.iter() {
        frame.push_bulk(Bytes::copy_from_slice(cm));
    }

    frame.push_int(tc.merkle_rt_count);

    for merkle_rt in tc.merkle_rts.iter() {
        frame.push_bulk(Bytes::copy_from_slice(merkle_rt));
    }

    frame.push_bulk(Bytes::from(tx_hash));
}

pub(crate) fn put_pour_tx_into_frame(frame: &mut Frame, tx: PourTx) {
    let tc = tx.tx_candidate;

    let cm_count = tc.cm_count;

    put_pour_tx_candidate_into_frame(frame, tc);

    frame.push_int(cm_count);

    for cm_idx in tx.cm_idxes {
        frame.push_int(cm_idx);
    }
}
