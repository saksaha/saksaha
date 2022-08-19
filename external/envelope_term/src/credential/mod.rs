pub(crate) struct Credential {
    pub public_key: String,
    pub secret: String,
    pub acc_addr: String,
}

impl Credential {
    pub fn new(
        public_key: Option<String>,
        secret: Option<String>,
    ) -> Credential {
        let public_key = public_key.unwrap_or("public_key".to_string());
        let secret = secret.unwrap_or("secret".to_string());
        let acc_addr = String::from("acc_addr");

        Credential {
            public_key,
            secret,
            acc_addr,
        }
    }
}
