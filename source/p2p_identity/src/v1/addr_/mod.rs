mod known;
mod status;
mod unknown;

pub use known::*;
pub use status::*;
pub use unknown::*;

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
