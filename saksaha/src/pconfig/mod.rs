use crate::{
    common::errors::Error,
    crypto,
};
use directories::ProjectDirs;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use logger::log;
use std::path::{Path, PathBuf};

pub mod parse;

pub mod fs;

pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

pub struct PersistedP2PConfig {
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

// impl PartialEq for Book {
//     fn eq(&self, other: &Self) -> bool {
//         self.isbn == other.isbn
//     }
// }

impl PConfig {

    pub fn new(path: Option<&str>) -> Result<Self, Error> {
        let sk = crypto::generate_key();
        let pk = sk.public_key();

        crypto::encode_hex(sk.to_bytes().as_slice());
        let bb = pk.to_encoded_point(false);

        // crypto::encode_hex(pk.as_affine());
        return Error::result(format!("f"));
    }
}
