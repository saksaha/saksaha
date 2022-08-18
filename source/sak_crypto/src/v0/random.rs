use rand::prelude::*;

pub fn rand() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen::<usize>() % 1000000
}
