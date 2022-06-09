use std::path::{Path, PathBuf};

const DEPENDENCIES: [&str; 3] = ["wasm-pack", "clang", "wasm2wat"];

const CONTRACTS: [&str; 1] = ["validators"];

fn main() {
    check_dependencies();

    // build_sys_contracts();
}

fn check_dependencies() {
    for d in DEPENDENCIES {
        match find_in_env_path(d) {
            Some(_) => {}
            None => {
                panic!("{} is needed as a dependency", d);
            }
        };
    }
}

// fn build_sys_contracts() {
//     let sys_contracts_path = {
//         let p = Path::new("src/blockchain/sys_contracts");

//         if !p.exists() {
//             panic!("power");
//         }

//         p
//     };

//     for file in sys_contracts_path
//         .read_dir()
//         .expect("sys_contracts_path should be open")
//     {}
// }

fn find_in_env_path<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}
