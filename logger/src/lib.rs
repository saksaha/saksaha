use std::str;

pub fn make_fd(f: &str, ln: u32) -> String {
    let s = format!("{}:{}", f, ln);
    let s = s.as_bytes();
    let mut fd: [u8; 18] = [b'.'; 18];

    for i in 0..16 {
        fd[fd.len() - 1 - i] = s[s.len() - 1 - i];
    }

    let fd = str::from_utf8(&fd).unwrap().to_string();
    fd
}

#[macro_export]
macro_rules! log {
    (DEBUG, $str_format: expr) => {
        {
            const f: &str = file!();
            const ln: u32 = line!();
            let fd = $crate::make_fd(f, ln);

            print!("{}  {}", fd, format_args!($str_format));
        }
    };

    (DEBUG, $str_format: expr, $($arg:tt)*) => {
        {
            const f: &str = std::file!();
            const ln: u32 = std::line!();
            let fd = $crate::make_fd(f, ln);
            print!("{}  {}", fd, format_args!($str_format, $($arg)*));
        }
    };
}
