use crate::{fs::SaksahaFS, SaksahaError};
use sak_mrs::{SakMRS, SakMRSArgs};

pub(crate) struct MRS {}

impl MRS {
    pub(crate) async fn init(public_key: &String) -> Result<SakMRS, SaksahaError> {
        let mrs_db_path = {
            let acc_dir = SaksahaFS::acc_dir(public_key)?;
            acc_dir.join("mrs")
        };

        let mrs_args = SakMRSArgs { mrs_db_path };

        let sak_ledger = SakMRS::init(mrs_args).await?;

        Ok(sak_ledger)
    }
}
