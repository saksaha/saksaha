use sak_types::CoinRecord;

use crate::{Config, WalletError};

pub fn dev_local_1(saksaha_endpoint: String) -> Result<Config, WalletError> {
    let c = Config {
        public_key: Some(String::from(
            "045739d074b8722891c307e8e75c9607e0b55a80778\
                b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
                e95590616fd382662a09653f2a966ff524989ae8c0f",
        )),
        secret: Some(String::from(
            "7297b903877a957748b74068d63d6d5661481975240\
                99fc1df5cd9e8814c66c7",
        )),
        coin_records: Some(vec![
            //
            CoinRecord::new(
                0x11,
                0x12,
                0x13,
                0x14,
                100,
                Some(0),
                None,
                Some(
                    "\
                ed763fdfcdb6146d0a172363f08b6a113\
                05cad7d78abe0c07aff9ea6369b08a8"
                        .to_string(),
                ),
            )?,
            CoinRecord::new(
                0x21,
                0x22,
                0x23,
                0x24,
                100,
                Some(1),
                None,
                Some(
                    "\
                8d526423d7f7e2c1d419c3096ecee5a8\
                f24c9269f018bfae35216858c19bbae1"
                        .to_string(),
                ),
            )?,
        ]),
        rpc_port: Some(36612),
        saksaha_endpoint: Some(format!(
            "http://localhost:{}/rpc/v0",
            saksaha_endpoint
        )),
        // saksaha_endpoint: Some(String::from("http://localhost:34418/rpc/v0")),
    };

    Ok(c)
}

pub fn dev_local_2(saksaha_endpoint: String) -> Result<Config, WalletError> {
    let c = Config {
        public_key: Some(String::from(
            "042c8d005bd935597117181d8ceceaef6d1162de78c32856\
                89d0c36c6170634c124f7b9b911553a1f483ec565c199ea29ff1\
                cd641f10c9a5f8c7c4d4a026db6f7b",
        )),
        secret: Some(String::from(
            "224d0898389759f29ad5c9a6472b26fff86b6293889\
                88eec457a88ce50e907a0",
        )),
        coin_records: None,
        // coin_records: Some(vec![
        //     //
        //     CoinRecord::new(
        //         0x21,
        //         0x22,
        //         0x23,
        //         0x24,
        //         100,
        //         Some(1),
        //         None,
        //         Some(
        //             "\
        //         8d526423d7f7e2c1d419c3096ecee5a8\
        //         f24c9269f018bfae35216858c19bbae1"
        //                 .to_string(),
        //         ),
        //     )?,
        // ]),
        rpc_port: Some(36613),
        saksaha_endpoint: Some(format!(
            "http://localhost:{}/rpc/v0",
            saksaha_endpoint
        )),
        // saksaha_endpoint: Some(String::from("http://localhost:34418/rpc/v0")),
    };

    Ok(c)
}
