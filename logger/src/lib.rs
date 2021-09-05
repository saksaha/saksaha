pub struct Log;

static mut DEFAULT_LOGGER: Log = Log {};

impl Log {
    pub fn new() {}

    pub fn debug(&self) {
        print!("444\n");
    }
}

pub fn add_one(x: i32) -> i32 {
    return x + 1;
}

#[macro_export]
macro_rules! log {
    ($($addend: expr),+) => {
        // static const a: i32 = 3;
        let mut sum = 0;
        $(sum += $addend;)*
        std::println!("{}", 3);
        println!("Sum: {}", sum);
    }

    // ($(#[$attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
    //     // use `()` to explicitly forward the information about private items
    //     __lazy_static_internal!($(#[$attr])* () static ref $N : $T = $e; $($t)*);
    // };
}

// #[macro_export]
// macro_rules! format {
//     ($($arg:tt)*) => {{
//         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
//         res
//     }}
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
