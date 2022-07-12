use crate::{TrptError, BLOCK_SYN_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use sak_types::{Block, MintTx, MintTxCandidate, PourTx, PourTxCandidate, Tx};

pub(crate) fn parse_mint_tx(parse: &mut Parse) -> Result<Tx, TrptError> {
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
        std::str::from_utf8(p.as_ref())?.into()
    };

    let v = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let k = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let s = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let _tx_hash: String = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let tx_height = parse.next_int()? as u128;

    let mint_tx = MintTxCandidate::new(
        created_at,
        data,
        author_sig,
        Some(ctr_addr),
        cm,
        v,
        k,
        s,
    );

    Ok(mint_tx.upgrade(tx_height))
}

pub(crate) fn parse_pour_tx(parse: &mut Parse) -> Result<Tx, TrptError> {
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
        b.to_vec()
    };

    let sn_2 = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let cm_1 = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let cm_2 = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let merkle_rt = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let _tx_hash: String = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let tx_height = parse.next_int()? as u128;

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

    Ok(pour_tx.upgrade(tx_height))
}

pub(crate) fn put_mint_tx_into_frame(frame: &mut Frame, tx: &MintTx) {
    let tc = tx.tx_candidate;

    frame.push_bulk(Bytes::from(tc.data));
    frame.push_bulk(Bytes::from(tc.created_at));
    frame.push_bulk(Bytes::from(tc.author_sig));
    frame.push_bulk(Bytes::from(tc.ctr_addr));
    frame.push_bulk(Bytes::from(tc.cm));
    frame.push_bulk(Bytes::from(tc.v));
    frame.push_bulk(Bytes::from(tc.k));
    frame.push_bulk(Bytes::from(tc.s));
    frame.push_bulk(Bytes::from(tc.get_tx_hash().to_string()));
    frame.push_int(tx.tx_height as u128);
}

pub(crate) fn put_pour_tx_into_frame(frame: &mut Frame, tx: &PourTx) {
    let tc = tx.tx_candidate;

    frame.push_bulk(Bytes::from(tc.data));
    frame.push_bulk(Bytes::from(tc.created_at));
    frame.push_bulk(Bytes::from(tc.author_sig));
    frame.push_bulk(Bytes::from(tc.ctr_addr));
    // frame.push_bulk(Bytes::from(tc.cm));
    // frame.push_bulk(Bytes::from(tc.v));
    // frame.push_bulk(Bytes::from(tc.k));
    // frame.push_bulk(Bytes::from(tc.s));
    frame.push_bulk(Bytes::from(tc.get_tx_hash().to_string()));
    frame.push_int(tx.tx_height as u128);
}
