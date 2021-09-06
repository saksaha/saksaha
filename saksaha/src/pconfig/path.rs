use crate::errors::Error;
use directories::ProjectDirs;
use logger::log;
use std::fs::{self};
use std::path::Path;

pub fn default_path() -> Result<bool, Error> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Saksaha", "Saksaha") {
        let p = proj_dirs.config_dir();
        let pstr = p.to_str().unwrap_or("");

        if pstr == "" {
            let err = Error::new(format!("Error converting path to string"));
            return Err(err);
        }

        if p.exists() {
            log!(DEBUG "Found a config path at: {}", p.to_str().unwrap());
            // load
        } else {
            log!(DEBUG "Couldn't find a config path, creating one...");

            let mut dir = fs::create_dir(p);

            match dir {
                Ok(_) => {
                    log!(DEBUG "Created a config path at: {}", pstr);
                }
                Err(_e) => {
                    print!("power11111\n")
                }
            }
        }
        // let some_number = Some(9);
        return Ok(true);
    } else {
        print!("{}\n", 123123);
        return Ok(true);
        // return io::Error::new();
    }
    // directories::config_dir()
}

#[cfg(test)]
mod test {
    use crate::testenv;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn it_creates_default_config_path() {
        testenv::run_test(|env, env2| {
            let testdump = PathBuf::from("_testdump");
            // let parent = Path::new(file!()).parent().expect(
            //     "Error cannot retrieve the parent path of the test file",
            // );
            // // .join("_testdump");

            // let d = Path::new("src/pconfig/_testdump");
            // println!("11, {:?} {}\n", d.exists(), env!("CARGO_MANIFEST_DIR"));

            // // let p = PathBuf::new("..");
            // let x = std::env::current_dir().unwrap();
            // let xx = env!("CARGO_MANIFEST_DIR");
            // print!("33 {:?}, {} {} \n", x, file!(), xx);
            // let b = fs::canonicalize("./_testdump");
            // println!("55 {:?}", b);

            // // let aa = parent.to_s

            // // Path::new(f);
            // // Path::new(f).parent();
            // let a = Path::new(file!()).join("../_testdump");
            // let b = a.to_str().unwrap();

            // let c = parent.exists();
            // print!("444 {}\n", c)
        })
    }

    #[test]
    fn it_another() {
        testenv::run_test(|_, _| {
            print!("123123123\n");
        });
    }
}
