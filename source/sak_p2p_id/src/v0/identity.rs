use crate::Credential;

pub struct Identity {
    pub credential: Credential,
    pub p2p_port: u16,
    pub disc_port: u16,
}

impl Identity {
    pub fn new(
        secret: String,
        public_key_str: String,
        p2p_port: u16,
        disc_port: u16,
    ) -> Result<Identity, String> {
        let credential = Credential::new(secret, public_key_str)?;

        let i = Identity {
            credential,
            p2p_port,
            disc_port,
        };

        Ok(i)
    }
}
