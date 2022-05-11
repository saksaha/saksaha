mod known;
mod unknown;

pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
    EncodedPoint, PublicKey, SecretKey,
};
pub use known::KnownAddr;
use serde::{Deserialize, Serialize};
pub use unknown::UnknownAddr;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub enum Addr {
//     Unknown(UnknownAddr),

//     #[serde(skip)]
//     Known(KnownAddr),
// }

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

// impl Addr {
//     pub fn disc_endpoint(&self) -> String {
//         match self {
//             Addr::Known(k) => k.disc_endpoint(),
//             Addr::Unknown(u) => u.disc_endpoint(),
//         }
//     }
// }

impl std::fmt::Display for UnknownAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ip: {}, disc_port: {}, p2p_port: {:?}",
            self.ip, self.disc_port, self.p2p_port,
        )
    }
}
