use super::utils::MRSTestUtils;
use crate::v0::testing;
use sak_types::{Block, BlockCandidate};

pub(crate) const CONTRACT: &[u8] = include_bytes!("../../../../prebuild/sak_mrs.postprocess.wasm");

#[tokio::test(flavor = "multi_thread")]
async fn test_reserve_slot() {
    MRSTestUtils::init_test(vec!["test"]);
}
