pub const USER_1: &str = "user_1";

pub const USER_2: &str = "user_2";

pub(crate) mod cfs {
    // my_sk => my_pk
    // my_sk => my_sig

    // ch id => aes_key
    // ch id => her_pk

    // ch_id can be obtained from get_ch_list(my_pk)
    pub const MY_SK: &str = "my_sk";

    pub const MY_PK: &str = "my_pk";

    pub const MY_SIG: &str = "my_sig";

    pub const CH_ID: &str = "ch_id";

    pub const HER_PK: &str = "her_pk";

    pub const AES_KEY: &str = "aes_key";

    pub const USER_ID: &str = "user_id";
}
