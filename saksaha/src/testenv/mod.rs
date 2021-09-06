use std::path::PathBuf;
use std::sync::Once;

static INIT: Once = Once::new();

// static mut TESTDUMP: PathBuf;

pub fn init() {
    INIT.call_once(|| {
        print!("242424\n");
    })
}

pub fn testdump() {

}

pub fn run_test<T>(test: T)
where
    T: FnOnce(u32, u32),
{
    print!("535353\n");
    init();
    test(3, 4);
}
