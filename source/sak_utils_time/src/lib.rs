use sak_logger::error;
use std::time::{Duration, SystemTime};

pub async fn wait_until_min_interval(time_since: SystemTime, min_interval: Duration) {
    match time_since.elapsed() {
        Ok(d) => {
            if d < min_interval {
                let diff = min_interval - d;
                tokio::time::sleep(diff).await;
            }
        }
        Err(err) => {
            error!("Calculating the time elapsed fail, err: {}", err);

            tokio::time::sleep(min_interval).await;
        }
    }
}
