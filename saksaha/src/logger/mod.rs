pub struct Log;

static mut DEFAULT_LOGGER: Log = Log {};

impl Log {
    pub fn new() {

    }

    pub fn debug(&self) {
        print!("444\n");
    }
}

// impl Logger {
//     pub fn default() -> Logger {
//         DEFAULT_LOGGER;
//     }
// }

#[macro_use]
pub fn get_instance() -> &'static Log {
    unsafe {
        // let a = &DEFAULT_LOGGER;
        // print!("{:p} 33\n", &DEFAULT_LOGGER);
        return &DEFAULT_LOGGER;
    }
    // log1!()
}

// #[macro_export]
// macro_rules! log1 {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }


// #[macro_use]
// macro_rules! bar {
//     () => ()
// }
