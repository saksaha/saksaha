use crate::{common::Result, err};

#[derive(Debug)]
pub struct Address {
    pub ip: String,
    pub disc_port: u16,
    pub peer_id: String,
    pub url: String,
}

impl Address {
    fn parse_full_url(url: String) -> Result<Address> {
        let (peer_id, ip, disc_port) = match url.get(6..) {
            Some(u) => match u.split_once('@') {
                Some((peer_id, endpoint)) => match endpoint.split_once(":") {
                    Some((ip, port)) => {
                        (peer_id.to_string(), ip.to_string(), port.to_string())
                    }
                    None => {
                        return err!("url may have illegal ip or port");
                    }
                },
                None => {
                    return err!("url is not valid, url: {}", url);
                }
            },
            None => {
                return err!("url might be of a short form, url: {}", url);
            }
        };

        let disc_port = match Address::parse_disc_port(disc_port) {
            Ok(d) => d,
            Err(err) => return Err(err),
        };

        Ok(Address {
            peer_id,
            ip,
            disc_port,
            url: url.clone(),
        })
    }

    fn parse_short_url(url: String) -> Result<Address> {

    }

    pub fn parse(url: String) -> Result<Address> {
        if url.starts_with("sak://") {
            return Address::parse_full_url(url)
        } else {
            return Address::parse_short_url(url)
        }

        // let (peer_id, ip, disc_port) = {
        //     match url.get(6..) {
        //         Some(u) => match u.split_once('@') {
        //             Some((peer_id, endpoint)) => {
        //                 match endpoint.split_once(":") {
        //                     Some((ip, port)) => (
        //                         peer_id.to_string(),
        //                         ip.to_string(),
        //                         port.to_string(),
        //                     ),
        //                     None => {
        //                         return err!("url may have illegal ip or port");
        //                     }
        //                 }
        //             }
        //             None => {
        //                 return err!("url is not valid, url: {}", url);
        //             }
        //         },
        //         None => {
        //             return err!("url might be of a short form, url: {}", url);
        //         }
        //     }
        // };

        // let disc_port = match disc_port.parse::<u16>() {
        //     Ok(d) => d,
        //     Err(err) => {
        //         return err!(
        //             "disc port cannot be converted to u16, err: {}, \
        //                 url: {}, disc_port: {}",
        //             err,
        //             url,
        //             disc_port,
        //         )
        //     }
        // };

        // let addr = Address {
        //     ip,
        //     disc_port,
        //     peer_id,
        //     url,
        // };

        // Ok(addr)
    }

    fn parse_disc_port(disc_port: String) -> Result<u16> {
        match disc_port.parse::<u16>() {
            Ok(d) => d,
            Err(err) => {
                return err!(
                    "disc port cannot be converted to u16, err: {}, \
                        disc_port: {}",
                    err,
                    disc_port,
                )
            }
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
