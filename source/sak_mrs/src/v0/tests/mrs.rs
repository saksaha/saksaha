use super::utils::MRSTestUtils;
use crate::v0::testing;
use sak_types::{Block, BlockCandidate};

#[tokio::test(flavor = "multi_thread")]
async fn test_something() {
    MRSTestUtils::init_test(vec!["test"]);
}
