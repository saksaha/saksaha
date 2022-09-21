use chrono::{DateTime, Duration, Local, Utc};

pub(crate) const WHO_ARE_YOU_EXPIRATION_SEC: i64 = 60;

pub(crate) fn is_my_endpoint(src_disc_port: u16, dest_endpoint: &String) -> bool {
    let my_disc_endpoint = format!("127.0.0.1:{}", src_disc_port);

    my_disc_endpoint == *dest_endpoint
}

pub(crate) fn is_who_are_you_expired(expiration: i64, known_at: DateTime<Utc>) -> bool {
    let now = Local::now();
    known_at.signed_duration_since(now) > Duration::seconds(expiration)
}
