use super::DBSchema;
use crate::db::cfs;
use crate::WalletError;

impl DBSchema {
    pub async fn get_status(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::STATUS)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_user_id(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::USER_ID)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_rho(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::RHO)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_r(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::R)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_s(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::S)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_v(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::V)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_a_pk(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_PK)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_a_sk(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_SK)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }
    // pub async fn get_aes_key(
    //     &self,
    //     ch_id: &String,
    // ) -> Result<Option<[u8; 32]>, WalletError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::AES_KEY)?;
    //     match self.db.get_cf(&cf, ch_id)? {
    //         Some(v) => {
    //             let str = String::from_utf8(v)?;
    //             let u8_arr = serde_json::from_str(&str)?;

    //             return Ok(Some(u8_arr));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     };
    // }
}
