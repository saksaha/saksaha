use crate::error::Error;

#[derive(Debug)]
pub struct Address {
    pub ip: String,
    pub disc_port: u16,
    pub peer_id: String,
    pub url: String,
}

impl Address {
    fn parse_full_url(url: String) -> Result<Address, Error> {
        let (peer_id, ip, disc_port) = match url.get(6..) {
            Some(u) => match u.split_once('@') {
                Some((peer_id, endpoint)) => {
                    let (ip, port) = parse_endpoint(endpoint)?;
                    (peer_id.to_string(), ip, port)
                }
                None => {
                    let msg = format!("url is not valid, url: {}", url);
                    return Err(Error::new(msg));
                }
            },
            None => {
                let msg = format!("url might be of a short form, url: {}", url);
                return Err(Error::new(msg));
            }
        };

        Ok(Address {
            peer_id,
            ip,
            disc_port,
            url,
        })
    }

    fn parse_short_url(url: String) -> Result<Address, Error> {
        let (ip, disc_port) = parse_endpoint(url.as_str())?;
        Ok(Address {
            peer_id: "".to_string(),
            ip,
            disc_port,
            url,
        })
    }

    pub fn parse(url: String) -> Result<Address, Error> {
        if url.starts_with("sak://") {
            return Address::parse_full_url(url);
        } else {
            return Address::parse_short_url(url);
        }
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
}

fn parse_endpoint(endpoint: &str) -> Result<(String, u16), Error> {
    if endpoint.matches(".").count() < 3 {
        let msg = format!(
            "endpoint may not have a valid ip address, endpoint: {}",
            endpoint
        );
        return Err(Error::new(msg));
    }

    match endpoint.split_once(":") {
        Some((ip, port)) => {
            let port = parse_port(port)?;
            Ok((ip.to_string(), port))
        }
        None => {
            let msg = format!(
                "Error splitting endpoint into ip and port, endpoint: {}",
                endpoint
            );
            return Err(Error::new(msg));
        }
    }
}

fn parse_port(port: &str) -> Result<u16, Error> {
    match port.parse::<u16>() {
        Ok(d) => Ok(d),
        Err(err) => {
            let msg = format!(
                "disc port cannot be converted to u16, err: {}, \
                    disc_port: {}",
                err,
                port,
            );

            return Err(Error::new(msg));
        }
    }
}
