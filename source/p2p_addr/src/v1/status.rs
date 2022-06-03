use chrono::{DateTime, Duration, Local, Utc};

#[derive(Debug, Clone)]
pub enum AddrStatus {
    Invalid { err: String },
    Initialized,
    WhoAreYouInProgress,
    WhoAreYouSuccess { at: DateTime<Utc> },
}

impl AddrStatus {
    pub fn is_registered_long_ago(&self, how_long: Duration) -> bool {
        if let AddrStatus::WhoAreYouSuccess { at } = self {
            let now = Local::now();
            return at.signed_duration_since(now) > how_long;
        }

        return false;
    }
}

impl Default for AddrStatus {
    fn default() -> Self {
        AddrStatus::Initialized
    }
}
