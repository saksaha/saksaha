use std::fs;
use log::{debug};
use std::path::{Path, PathBuf};
use std::sync::Once;

static INIT: Once = Once::new();

static mut TEST_ENV: TestEnv = TestEnv { testdump: None };

pub struct TestEnv {
    pub testdump: Option<PathBuf>,
}

fn init() {
    INIT.call_once(|| unsafe {
        debug!("Initializing the test environment. \
            This should be called once.");

        let p = PathBuf::from("testdump");

        if p.exists() {
            debug!(
                "Test folder already exists, replace it with a new empty one, \
                path: {}",
                p.to_str().unwrap(),
            );

            // let a = p.as_ref();
            fs::remove_dir_all(<PathBuf as AsRef<Path>>::as_ref(&p))
                .expect("Previous test folder cannot be removed");
        }

        fs::create_dir(<PathBuf as AsRef<Path>>::as_ref(&p))
            .expect("Test directory cannot be created");

        debug!(
            "Test directory is created, path: {}",
            p.to_str().unwrap(),
        );

        TEST_ENV.testdump = Some(p);
    })
}

/// This is still experimental.
/// At the moment, it doesn't do anything substantial.
pub fn run_test<T>(test: T)
where
    T: FnOnce(&TestEnv),
{
    init();
    unsafe {
        test(&TEST_ENV);
    }
}
