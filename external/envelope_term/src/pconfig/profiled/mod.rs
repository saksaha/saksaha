use crate::EnvelopeError;

use super::PConfig;

pub fn dev_local_1() -> Result<PConfig, EnvelopeError> {
    let c = PConfig {
        user_id: Some(String::from("user_1")),
        public_key: Some(String::from(
            "045739d074b8722891c307e8e75c9607e0b55a80778\
                b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
                e95590616fd382662a09653f2a966ff524989ae8c0f",
        )),
        secret: Some(String::from(
            "7297b903877a957748b74068d63d6d5661481975240\
                99fc1df5cd9e8814c66c7",
        )),
    };

    Ok(c)
}

pub fn dev_local_2() -> Result<PConfig, EnvelopeError> {
    let c = PConfig {
        user_id: Some(String::from("user_2")),
        public_key: Some(String::from(
            "042c8d005bd935597117181d8ceceaef6d1162de78c32856\
                89d0c36c6170634c124f7b9b911553a1f483ec565c199ea29ff1\
                cd641f10c9a5f8c7c4d4a026db6f7b",
        )),
        secret: Some(String::from(
            "224d0898389759f29ad5c9a6472b26fff86b6293889\
                88eec457a88ce50e907a0",
        )),
    };

    Ok(c)
}
