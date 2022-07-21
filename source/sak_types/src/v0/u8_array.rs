pub struct U8Array;

impl U8Array {
    pub fn new_empty_32() -> [u8; 32] {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    }

    pub fn from_int(v: u64) -> [u8; 32] {
        let arr: [u8; 8] = v.to_le_bytes();

        let mut ret = [0u8; 32];

        let _ = &ret[24..].copy_from_slice(&arr);

        ret
    }
}
