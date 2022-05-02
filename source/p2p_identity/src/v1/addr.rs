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

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum Addr {
//     Known(KnownAddr),
//     Unknown(UnknownAddr),
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Addr {
    pub ip: String,
    pub disc_port: u16,
    // pub p2p_port: u16,
    // pub secret: String,
    // pub public_key: String,
    pub p2p_port: Option<u16>,

    #[serde(skip)]
    pub sig: Option<Signature>,
    pub public_key: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct UnknownAddr {
//     pub ip: String,
//     pub disc_port: u16,
//     // pub p2p_port: Option<u16>,
//     // pub secret: Option<String>,
//     // pub public_key: Option<String>,
// }

// impl Addr {
//     pub fn disc_endpoint(&self) -> String {
//         match &*self {
//             Self::Known(known_addr) => known_addr.disc_endpoint(),
//             Self::Unknown(unknown_addr) => unknown_addr.disc_endpoint(),
//         }
//     }
// }

// impl Addr {
//     pub fn disc_endpoint(&self) -> String {
//         disc_endpoint(&self.ip, self.disc_port)
//     }
// }

impl Addr {
    pub fn new_from_url(url: String) -> Result<Addr, String> {
        if url.starts_with("sak://") {
            return Addr::new_from_full_url(url);
        } else {
            return Addr::new_from_short_url(url);
        }
    }

    pub fn new_from_socket_addr(addr: SocketAddr) -> Addr {
        Addr {
            ip: addr.ip().to_string(),
            disc_port: addr.port(),
            p2p_port: None,
            sig: None,
            public_key: None,
        }
    }

    fn new_from_full_url(url: String) -> Result<Addr, String> {
        let (peer_id, ip, disc_port) = match url.get(6..) {
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

        Ok(Addr {
            ip,
            disc_port,
            p2p_port: None,
            sig: None,
            public_key: None,
        })
    }

    fn new_from_short_url(url: String) -> Result<Addr, String> {
        let (ip, disc_port) = parse_endpoint(url.as_str())?;

        Ok(Addr {
            ip,
            disc_port,
            p2p_port: None,
            sig: None,
            public_key: None,
        })
    }

    pub fn disc_endpoint(&self) -> String {
        disc_endpoint(&self.ip, self.disc_port)
        // format!("{}:{}", self.ip, self.disc_port)
    }

    pub fn short_url(&self) -> String {
        let peer_id_short = {
            if let Some(pid) = &self.public_key {
                &pid[..6]
            } else {
                "[Unknown]"
            }
        };

        format!("{}@{}:{}", peer_id_short, self.ip, self.disc_port)
    }
}

fn disc_endpoint(ip: &String, disc_port: u16) -> String {
    format!("{}:{}", ip, disc_port)
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
