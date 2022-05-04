mod known;

pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
    EncodedPoint, PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub use self::known::KnownAddr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Addr {
    Unknown(UnknownAddr),

    #[serde(skip)]
    Known(KnownAddr),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnknownAddr {
    pub ip: String,
    pub disc_port: u16,
    pub p2p_port: Option<u16>,
    #[serde(skip)]
    pub sig: Option<Signature>,
    pub public_key_str: Option<String>,
}

impl UnknownAddr {
    pub fn new_from_url(url: String) -> Result<UnknownAddr, String> {
        if url.starts_with("sak://") {
            return UnknownAddr::new_from_full_url(url);
        } else {
            return UnknownAddr::new_from_short_url(url);
        }
    }

    pub fn new_from_socket_addr(addr: SocketAddr) -> UnknownAddr {
        UnknownAddr {
            ip: addr.ip().to_string(),
            disc_port: addr.port(),
            p2p_port: None,
            sig: None,
            public_key_str: None,
        }
    }

    fn new_from_full_url(url: String) -> Result<UnknownAddr, String> {
        let (public_key_str, ip, disc_port) = match url.get(6..) {
            Some(u) => match u.split_once('@') {
                Some((peer_id, endpoint)) => {
                    let (ip, port) = parse_endpoint(endpoint)?;
                    (peer_id.to_string(), ip, port)
                }
                None => {
                    return Err(format!("url is not valid, url: {}", url));
                }
            },
            None => {
                return Err(format!(
                    "url might be of a short form, url: {}",
                    url
                ));
            }
        };

        Ok(UnknownAddr {
            ip,
            disc_port,
            p2p_port: None,
            sig: None,
            public_key_str: Some(public_key_str),
        })
    }

    fn new_from_short_url(url: String) -> Result<UnknownAddr, String> {
        let (ip, disc_port) = parse_endpoint(url.as_str())?;

        Ok(UnknownAddr {
            ip,
            disc_port,
            p2p_port: None,
            sig: None,
            public_key_str: None,
        })
    }

    pub fn disc_endpoint(&self) -> String {
        make_endpoint(&self.ip, self.disc_port)
    }

    pub fn p2p_endpoint(&self) -> Option<String> {
        match self.p2p_port {
            Some(p) => Some(make_endpoint(&self.ip, p)),
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

fn make_endpoint(ip: &String, port: u16) -> String {
    format!("{}:{}", ip, port)
}

fn parse_endpoint(endpoint: &str) -> Result<(String, u16), String> {
    if endpoint.matches(".").count() < 3 {
        return Err(format!(
            "endpoint may not have a valid ip address, endpoint: {}",
            endpoint
        ));
    }

    match endpoint.split_once(":") {
        Some((ip, port)) => {
            let port = parse_port(port)?;
            Ok((ip.to_string(), port))
        }
        None => {
            return Err(format!(
                "Error splitting endpoint into ip and port, endpoint: {}",
                endpoint
            ));
        }
    }
}

fn parse_port(port: &str) -> Result<u16, String> {
    match port.parse::<u16>() {
        Ok(d) => Ok(d),
        Err(err) => {
            return Err(format!(
                "disc port cannot be converted to u16, err: {}, \
                    disc_port: {}",
                err, port,
            ));
        }
    }
}

impl Addr {
    pub fn disc_endpoint(&self) -> String {
        match self {
            Addr::Known(k) => k.disc_endpoint(),
            Addr::Unknown(u) => u.disc_endpoint(),
        }
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
