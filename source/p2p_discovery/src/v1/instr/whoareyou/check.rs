use p2p_identity::addr::Addr;

pub(crate) fn is_my_endpoint(src_disc_port: u16, dest_addr: &Addr) -> bool {
    let my_disc_endpoint = format!("127.0.0.1:{}", src_disc_port);
    let dest_disc_endpoint = dest_addr.disc_endpoint();

    my_disc_endpoint == dest_disc_endpoint
}
