pub mod peer_store;

use logger::log;

const MAX_FAIL_COUNT: usize = 3;

#[derive(Debug, PartialEq)]
pub enum Status {
    Empty,

    NotInitialized,

    DiscoverySuccess,

    HandshakeSuccess,

    HandshakeFail(String),
}

#[derive(Debug)]
pub struct Peer {
    pub ip: String,
    pub disc_port: u16,
    pub peer_op_port: u16,
    pub public_key_bytes: [u8; 65],
    pub rpc_port: u16,
    pub peer_id: String,
    pub status: Status,
    pub fail_count: usize,
    pub url: String,
}

impl Peer {
    pub fn new_empty() -> Peer {
        Peer {
            ip: "".into(),
            disc_port: 0,
            peer_op_port: 0,
            public_key_bytes: [0; 65],
            rpc_port: 0,
            peer_id: "".into(),
            status: Status::Empty,
            fail_count: 0,
            url: "".into(),
        }
    }

    pub fn parse(url: String) -> Result<Peer, String> {
        let (peer_id, ip, disc_port) = {
            match url.get(6..) {
                Some(u) => match u.split_once('@') {
                    Some((peer_id, endpoint)) => {
                        match endpoint.split_once(":") {
                            Some((ip, port)) => (
                                peer_id.to_string(),
                                ip.to_string(),
                                port.to_string(),
                            ),
                            None => {
                                return Err(format!(
                                    "url may have illegal ip or port"
                                ));
                            }
                        }
                    }
                    None => {
                        return Err(format!("url is not valid, url: {}", url));
                    }
                },
                None => {
                    return Err(format!(
                        "url might be too short, url: {}",
                        url
                    ));
                }
            }
        };

        let disc_port = match disc_port.parse::<u16>() {
            Ok(d) => d,
            Err(err) => {
                return Err(format!(
                    "disc port cannot be converted to u16, err: {}",
                    err
                ));
            }
        };

        let peer = Peer {
            ip,
            disc_port,
            peer_op_port: 0,
            public_key_bytes: [0; 65],
            rpc_port: 0,
            peer_id,
            status: Status::NotInitialized,
            fail_count: 0,
            url,
        };

        Ok(peer)
    }

    pub fn short_url(&self) -> String {
        let peer_id_short = {
            if self.peer_id.len() > 6 {
                &self.peer_id[..6]
            } else {
                ".."
            }
        };

        format!("{}@{}:{}", peer_id_short, self.ip, self.disc_port)
    }

    pub fn empty(&mut self) {
        *self = Peer::new_empty();
    }

    pub fn record_fail(&mut self) {
        self.fail_count += 1;

        if self.fail_count >= MAX_FAIL_COUNT {
            log!(
                DEBUG,
                "Peer fail count reached max, count: {}, peer: {}\n",
                self.fail_count,
                self.short_url()
            );

            self.empty();
        }
    }
}
