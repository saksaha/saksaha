use log::info;

pub(crate) struct Credential {
    pub id: String,
    pub key: String,
}

const DEFAULT_ID: &'static str = "default_user";
const DEFAULT_KEY: &'static str = "default_key";

impl Credential {
    pub fn new(id: Option<String>, key: Option<String>) -> Credential {
        let id = id.unwrap_or_else(|| {
            info!("Id is not specified, defaults to '{}'", DEFAULT_ID);

            return DEFAULT_ID.to_string();
        });

        let key = key.unwrap_or_else(|| {
            info!("Key is not specified, defaults to '{}'", DEFAULT_KEY);

            return DEFAULT_KEY.to_string();
        });

        Credential { id, key }
    }
}
