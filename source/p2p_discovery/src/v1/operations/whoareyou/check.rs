pub fn is_my_endpoint(my_disc_port: u16, endpoint: &String) -> bool {
    let my_disc_endpoint = format!("127.0.0.1:{}", my_disc_port);

    my_disc_endpoint == *endpoint
}
