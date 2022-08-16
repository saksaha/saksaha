pub struct U8Array;

pub type U8Arr32 = [u8; 32];

impl U8Array {
    pub fn new_empty_32() -> U8Arr32 {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    }

    pub fn from_int(v: u64) -> U8Arr32 {
        let arr: [u8; 8] = v.to_le_bytes();

        let mut ret = [0u8; 32];

        let _ = &ret[24..].copy_from_slice(&arr);

        println!("from_int(), v: {}, arr: {:?}", v, ret);

        ret
    }
}
