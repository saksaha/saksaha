use super::status::AddrStatus;
use sak_crypto::Signature;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnknownAddr {
    pub ip: String,
    pub disc_port: u16,
    pub p2p_port: Option<u16>,
    #[serde(skip)]
    pub sig: Option<Signature>,
    pub public_key_str: Option<String>,
    #[serde(skip)]
    pub status: AddrStatus,
}

impl UnknownAddr {
    pub fn new_from_url(url: String) -> Result<UnknownAddr, String> {
        if url.starts_with("sak://") {
            return UnknownAddr::new_from_full_url(url);
        } else {
            return UnknownAddr::new_from_short_url(url);
        }
    }

    pub fn new_from_endpoint(ip: &String, disc_port: u16) -> UnknownAddr {
        UnknownAddr {
            ip: ip.to_string(),
            disc_port,
            p2p_port: None,
            sig: None,
            public_key_str: None,
            status: AddrStatus::Initialized,
        }
    }

    fn new_from_full_url(url: String) -> Result<UnknownAddr, String> {
        let (public_key_str, ip, disc_port) = match url.get(6..) {
            Some(u) => match u.split_once('@') {
                Some((peer_id, endpoint)) => {
                    let (ip, port) = sak_utils_net::parse_endpoint(endpoint)?;
                    (peer_id.to_string(), ip, port)
                }
                None => {
                    return Err(format!("url is not valid, url: {}", url));
                }
            },
            None => {
                return Err(format!("url might be of a short form, url: {}", url));
            }
        };

        Ok(UnknownAddr {
            ip,
            disc_port,
            p2p_port: None,
            sig: None,
            public_key_str: Some(public_key_str),
            status: AddrStatus::Initialized,
        })
    }

    fn new_from_short_url(url: String) -> Result<UnknownAddr, String> {
        let (ip, disc_port) = sak_utils_net::parse_endpoint(url.as_str())?;

        Ok(UnknownAddr {
            ip,
            disc_port,
            p2p_port: None,
            sig: None,
            public_key_str: None,
            status: AddrStatus::Initialized,
        })
    }

    pub fn disc_endpoint(&self) -> String {
        sak_utils_net::make_endpoint(&self.ip, self.disc_port)
    }

    pub fn p2p_endpoint(&self) -> Option<String> {
        match self.p2p_port {
            Some(p) => Some(sak_utils_net::make_endpoint(&self.ip, p)),
            None => None,
        }
    }

    pub fn short_url(&self) -> String {
        let peer_id_short = {
            if let Some(pid) = &self.public_key_str {
                &pid[..6]
            } else {
                "[Unknown]"
            }
        };

        format!("{}@{}:{}", peer_id_short, self.ip, self.disc_port)
    }
}

impl std::fmt::Display for UnknownAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ip: {}, disc_port: {}, p2p_port: {:?}",
            self.ip, self.disc_port, self.p2p_port,
        )
    }
}
