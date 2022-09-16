use sak_types::CoinRecord;

use crate::{Config, WalletError};

pub fn dev_local_1() -> Result<Config, WalletError> {
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
                    "87b2f2ca4c9c22de99c3b4c550c2fd09\
                    906644aa735b823bcd0446921ddec498"
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
                    "5fdc798f16ae272047631de0e2d925a0\
                    83c39689cea69e5706a09224797fc99e"
                        .to_string(),
                ),
            )?,
            // Dummy Old Coin : value 0
            // CM : 3bb4c03f8e718ec58f4f2bb2b2fb83149b5fe59a75c5c98893e40c56bb3e8deb
            // CoinRecord::new(
            //     0x0,
            //     0x0,
            //     0x0,
            //     0x0,
            //     0,
            //     Some(2),
            //     None,
            //     Some(
            //         "\
            //     d71916a3daccd319e8256f892fcec0ca\
            //     fc65a1545cf55c9fc67f3c9ec7868fa2"
            //             .to_string(),
            //     ),
            // )?,
        ]),
        rpc_port: Some(36612),
        saksaha_endpoint: Some(String::from("http://localhost:34418/rpc/v0")),
    };

    Ok(c)
}

pub fn dev_local_2() -> Result<Config, WalletError> {
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
        coin_records: Some(vec![
            //
            CoinRecord::new(
                0x31,
                0x32,
                0x33,
                0x34,
                100,
                Some(4),
                None,
                Some(
                    "\
                ccded931042a124c36cf26f3ea8d2d58\
                57f96ecfd4c1b6bfa47f3911fd8c79e4"
                        .to_string(),
                ),
            )?,
            CoinRecord::new(
                0x41,
                0x42,
                0x43,
                0x44,
                100,
                Some(5),
                None,
                Some(
                    "\
                eb0907f91b7929e5454d75cac09ee314\
                3e185a75925c69443b7348b7b831728c"
                        .to_string(),
                ),
            )?,
        ]),
        rpc_port: Some(36613),
        saksaha_endpoint: Some(String::from("http://localhost:34418/rpc/v0")),
    };

    Ok(c)
}
