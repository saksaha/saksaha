use std::str;

const TAG_LEN: usize = 24;
const TAG_CONTENT_LEN: usize = 22;

pub fn make_fd(f: &str, ln: u32) -> String {
    let s = format!("{}:{}", f, ln);
    let s = s.as_bytes();
    let mut fd: [u8; TAG_LEN] = [b'.'; TAG_LEN];

    for i in 0..TAG_CONTENT_LEN {
        fd[fd.len() - 1 - i] = s[s.len() - 1 - i];
    }

    let fd = str::from_utf8(&fd).unwrap().to_string();
    fd
}

#[macro_export]
macro_rules! log {
    (DEBUG, $str_format: expr) => {
        #[cfg(debug_assertions)]
        {
                const f: &str = file!();
                const ln: u32 = line!();
                let fd = $crate::make_fd(f, ln);

                print!("{}  {}", fd, std::format_args!($str_format));
        }
    };

    (DEBUG, $str_format: expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            const f: &str = std::file!();
            const ln: u32 = std::line!();
            let fd = $crate::make_fd(f, ln);

            print!("{}  {}", fd, format_args!($str_format, $($arg)*));
        }
    };
}

#[test]
fn test_logger() {
    assert_eq!(1, 1);
}
