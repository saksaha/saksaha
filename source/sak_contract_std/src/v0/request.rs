use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request<'a> {
    pub ty: &'a str,
}
