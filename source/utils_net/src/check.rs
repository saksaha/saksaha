pub fn is_my_endpoint(src_port: u16, dest_endpoint: &String) -> bool {
    let my_endpoint = format!("127.0.0.1:{}", src_port);

    my_endpoint == *dest_endpoint
}
