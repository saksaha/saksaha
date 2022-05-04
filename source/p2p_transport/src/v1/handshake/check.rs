use p2p_identity::addr::Addr;

pub(crate) fn is_my_endpoint(src_port: u16, dest_addr: &Addr) -> bool {
    let my_endpoint = format!("127.0.0.1:{}", src_port);
    let dest_endpoint = dest_addr.disc_endpoint();

    my_endpoint == dest_endpoint
}
