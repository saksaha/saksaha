use rand::prelude::*;
use rand_core::OsRng;

pub fn rand() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen::<usize>() % 1000000
}

pub fn os_rng() -> OsRng {
    return OsRng;
}
