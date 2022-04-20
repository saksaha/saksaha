use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct KnownPeer {
    pub ip: String,
    pub disc_port: u16,
    pub p2p_port: u16,
    pub secret: String,
    pub public_key: String,
}

#[derive(Debug, Clone)]
pub struct UnknownPeer {
    pub ip: String,
    pub disc_port: u16,
    pub p2p_port: Option<u16>,
    pub secret: Option<String>,
    pub public_key: Option<String>,
}

impl UnknownPeer {
    // pub fn parse(url: String) -> UnknownPeer {
    //     UnknownPeer {}
    // }

    pub fn parse(url: String) -> Result<UnknownPeer, String> {
        if url.starts_with("sak://") {
            return UnknownPeer::parse_full_url(url);
        } else {
            return UnknownPeer::parse_short_url(url);
        }
    }

    pub fn from_socket_addr(addr: SocketAddr) -> UnknownPeer {
        UnknownPeer {
            ip: addr.ip().to_string(),
            disc_port: addr.port(),
            p2p_port: None,
            secret: None,
            public_key: None,
        }
    }

    fn parse_full_url(url: String) -> Result<UnknownPeer, String> {
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

        Ok(UnknownPeer {
            ip,
            disc_port,
            p2p_port: None,
            secret: None,
            public_key: None,
        })
    }

    fn parse_short_url(url: String) -> Result<UnknownPeer, String> {
        let (ip, disc_port) = parse_endpoint(url.as_str())?;

        Ok(UnknownPeer {
            ip,
            disc_port,
            p2p_port: None,
            secret: None,
            public_key: None,
        })
    }

    pub fn disc_endpoint(&self) -> String {
        format!("{}:{}", self.ip, self.disc_port)
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
