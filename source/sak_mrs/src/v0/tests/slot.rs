use super::utils::MRSTestUtils;
use crate::v0::db::{MrsEntity, MRSDB};

#[tokio::test(flavor = "multi_thread")]
async fn test_get_and_put_mrs_entity() {
    MRSTestUtils::init_test(vec!["test"]);
    let mrs_db = match MRSDB::init(&std::path::PathBuf::from("test")) {
        Ok(a) => a,
        Err(_) => panic!("panic"),
    };

    // mrs_db.
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_auto_incremental_indexing() {
//     MRSTestUtils::init_test(vec!["test"]);
//     let mrs_db = MRSDB::init(&std::path::PathBuf::from("test")).unwrap();
// }
