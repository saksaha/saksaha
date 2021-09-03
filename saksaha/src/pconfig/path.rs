use crate::common::errors::Error;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::{env, fs, io};

pub fn default_path() -> Result<bool, Error> {
    let os = env::consts::OS;

    println!("{}", os);

    if let Some(proj_dirs) = ProjectDirs::from("com", "Saksaha", "Saksaha") {
        let p = proj_dirs.config_dir();
        if p.exists() {
            // load
        } else {

            let mut dir = fs::create_dir(p);
            match dir {
                OK => {
                    print!("power\n")
                },
                Err(e) => {
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
