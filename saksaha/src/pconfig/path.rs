use crate::errors::Error;
use directories::ProjectDirs;
use logger::log;
use std::fs::{self};

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
