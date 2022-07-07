use blake2s_simd::{Hash, Params as Blake2sParams};

pub(crate) fn blake2s_fn(data: &[u8]) -> Hash {
    let mut h = Blake2sParams::new()
        .hash_length(32)
        .personal(b"12345678")
        .to_state();

    h.update(data);
    h.finalize()
}
