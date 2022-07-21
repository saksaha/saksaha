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

    let cm = {
        let p = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(p)?
    };

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
        cm,
        v,
        k,
        s,
    );

    Ok(mint_tx_candidate)
}

pub(crate) fn parse_mint_tx(parse: &mut Parse) -> Result<Tx, TrptError> {
    let mint_tx_candidate = parse_mint_tx_candidate(parse)?;

    let tx_height = parse.next_int()? as u128;

    Ok(mint_tx_candidate.upgrade(tx_height))
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

    let sn_1 = {
        let b = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(b)?
    };

    let sn_2 = {
        let b = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(b)?
    };

    let cm_1 = {
        let b = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(b)?
    };

    let cm_2 = {
        let b = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(b)?
    };

    let merkle_rt = {
        let b = parse.next_bytes()?;

        utils::convert_bytes_into_u8_32(b)?
    };

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
        sn_1,
        sn_2,
        cm_1,
        cm_2,
        merkle_rt,
    );

    Ok(pour_tx)
}

pub(crate) fn parse_pour_tx(parse: &mut Parse) -> Result<Tx, TrptError> {
    let pour_tx_candidate = parse_pour_tx_candidate(parse)?;

    let tx_height = parse.next_int()? as u128;

    Ok(pour_tx_candidate.upgrade(tx_height))
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
    frame.push_bulk(Bytes::copy_from_slice(&tc.cm));
    frame.push_bulk(Bytes::copy_from_slice(&tc.v));
    frame.push_bulk(Bytes::copy_from_slice(&tc.k));
    frame.push_bulk(Bytes::copy_from_slice(&tc.s));
    frame.push_bulk(Bytes::from(tx_hash));
}

pub(crate) fn put_mint_tx_into_frame(frame: &mut Frame, tx: MintTx) {
    let tc = tx.tx_candidate;

    put_mint_tx_candidate_into_frame(frame, tc);

    frame.push_int(tx.tx_height as u128);
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
    frame.push_bulk(Bytes::copy_from_slice(&tc.sn_1));
    frame.push_bulk(Bytes::copy_from_slice(&tc.sn_2));
    frame.push_bulk(Bytes::copy_from_slice(&tc.cm_1));
    frame.push_bulk(Bytes::copy_from_slice(&tc.cm_2));
    frame.push_bulk(Bytes::copy_from_slice(&tc.merkle_rt));
    frame.push_bulk(Bytes::from(tx_hash));
}

pub(crate) fn put_pour_tx_into_frame(frame: &mut Frame, tx: PourTx) {
    let tc = tx.tx_candidate;

    put_pour_tx_candidate_into_frame(frame, tc);

    frame.push_int(tx.tx_height as u128);
}
