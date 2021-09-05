use crate::common::errors::Error;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::{env, fs, io};

// use crate::logger::bar;
// use crate::loger::bar1;
// use crate::logger::bar1;
use logger::log;

// const v = log1!(1);


pub fn default_path() -> Result<bool, Error> {
    let os = env::consts::OS;

    log!(add 1,2);
    // print!("{}\n", v[0]);

    // log.debug();

    // println!("{}", os);

    if let Some(proj_dirs) = ProjectDirs::from("com", "Saksaha", "Saksaha") {
        let p = proj_dirs.config_dir();
        if p.exists() {
            // load
        } else {

            let mut dir = fs::create_dir(p);
            match dir {
                Ok(_v) => {
                    print!("power\n")
                },
                Err(_e) => {
                    print!("power11111\n")
                }
            }
        }
        // let some_number = Some(9);

        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        return Ok(true);
    } else {
        print!("{}\n", 123123);
        return Ok(true)
        // return io::Error::new();
    }
    // directories::config_dir()
}
