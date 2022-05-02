use p2p_identity::addr::Addr;

pub(crate) fn is_my_endpoint(disc_port: u16, addr: &Addr) -> bool {
    let my_disc_endpoint = format!("127.0.0.1:{}", disc_port);
    let her_disc_endpoint = addr.disc_endpoint();

    my_disc_endpoint == her_disc_endpoint
}
