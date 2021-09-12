use logger::log;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Once;

static INIT: Once = Once::new();

static mut TEST_ENV: TestEnv = TestEnv { testdump: None };

pub struct TestEnv {
    pub testdump: Option<PathBuf>,
}

fn init() {
    INIT.call_once(|| unsafe {
        log!(DEBUG, "Initializing the test environment. \
            This should be called once.\n");

        let p = PathBuf::from("testdump");

        if p.exists() {
            log!(
                DEBUG,
                "Test folder already exists, replace it with a new empty one, \
                path: {}\n",
                p.to_str().unwrap(),
            );

            // let a = p.as_ref();
            fs::remove_dir_all(<PathBuf as AsRef<Path>>::as_ref(&p))
                .expect("Previous test folder cannot be removed");
        }

        fs::create_dir(<PathBuf as AsRef<Path>>::as_ref(&p))
            .expect("Test directory cannot be created");

        log!(
            DEBUG,
            "Test directory is created, path: {}\n",
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
