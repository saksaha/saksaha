use clap;

pub struct Host {

}

pub struct Config {
    rpc_port: usize,
    bootstrap_peers: Vec<String>,
}

impl Host {
    pub fn new(
        conf: Config,
    ) -> Self {
        return Host {};
    }

    pub fn new_config(
        rpc_port: Option<&str>,
        bootstrap_peers: Option<clap::Values>,
        public_key: String,
        secret: String,
    ) -> Config {
        let mut c = Config {
            rpc_port: 0,
            bootstrap_peers: Vec::new(),
        };

        if let Some(p) = rpc_port {
            let rpc_port = p.parse::<usize>().unwrap();
            c.rpc_port = rpc_port;
        }

        if let Some(b) = bootstrap_peers {
            let bootstrap_peers = b.map(str::to_string).collect();
            c.bootstrap_peers = bootstrap_peers;
        }



        return c;
    }
}
