// pub(crate) mod cfs {
//     pub const MRS_KEY: &str = "mrs_key";

//     pub const MRS_VALUE: &str = "mrs_value";

//     pub const INTEGRITY_BITS: &str = "integrity_bits";

//     pub const TIMESTAMP: &str = "timestamp";

//     pub const IDX: &str = "idx";
// }

pub enum CFSenum {
    Slot = 0,
    Record = 1,
    RecordIdx = 2,
    RecordKey = 3,
    Idx = 4,
    // MrsValue = 1,
    // IntegrityBits = 2,
    // Timestamp = 3,

    // CMIdxByCM = 5,
    // BlockMerkleRt = 6,
    // EmptyValue = 7,
    // MerkleNode = 8,
    // BlockHash = 9,
    // CtrState = 10,
    // // test
    // MintTxEntity = 11,
    // PourTxEntity = 12,
    // BlockEntity = 13,
}

impl CFSenum {
    pub fn as_str(&self) -> &'static str {
        match self {
            CFSenum::Slot => "slot",
            CFSenum::Record => "record",
            CFSenum::RecordIdx => "record_idx",
            CFSenum::RecordKey => "record_key",
            CFSenum::Idx => "idx",
            // CFSenum::MrsValue => "mrs_value",
            // CFSenum::IntegrityBits => "integrity_bits",
            // CFSenum::Timestamp => "timestamp",
        }
    }
}
