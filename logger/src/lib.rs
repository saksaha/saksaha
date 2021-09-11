pub fn xx() -> &'static str {
    return file!();
}

#[macro_export]
macro_rules! log {
    (DEBUG, $str_format: expr) => {
        {
            const f: &[u8] = file!().as_bytes();
            const ln: u32 = line!();
            const ln_str: &str = stringify!(ln);
            const c: usize = f.len();
            const file_descriptor_size: usize = 12;
            // const f_len:
            // const ln_len: usize = ln.len();

            // // const f_bytes: &[u8] = f.as_bytes().len();
            // // const ln_bytes
            // // const fd: &[u8; file_descriptor_size] = &[b'.'; file_descriptor_size];
            let mut fd: [u8; file_descriptor_size] = [b'.'; file_descriptor_size];

            // fd[1] = b'a';

            // fd[0] = b'0';

            print!("{:?}\n", ln_str);

            // const ln_len: usize = ln_str.len();

            // for i in 0.. {
            //     print!("{}\n", f[i] as char);
            //     fd[file_descriptor_size - ln_len + i] = f[i];
            // }

            // fd[0] = b'3';

            // for i in 0..file_descriptor_size {
            //     print!("i {} {}\n", i, fd[i] as char);
            // }

            // for i in 0..ln_len {
            //     fd[ln[ln_len - 1 - i]
            // }
            // for i in (0..file_descriptor_size) {
            //     if i <
            //     print!("i {} {}\n", i, e[i] as char)
            //     // f
            //     // print!("55 {} {}\n", n, e[n] as char);
            // }
            // print!("{:?}\n", fd[0] as char);

            // for d in a.chars() {
            //     print!("555 {}\n", d);
            // }

            // const a: &str = file!();
            // const b: &str = line!();
            // const c: String = format!("{}{}", a, b);
            // print!("44 {}\n", c);
            // let fln: String = format!("{}:{}", f, ln);

            // let a: usize = fln.as_bytes().len();

            // print!("{}:{} {}", f, ln, format_args!($str_format));
            ()
        }
    };

    (DEBUG, $str_format: expr, $($arg:tt)*) => {
        {
            const f: &str = std::file!();
            const ln: u32 = std::line!();
            print!("{}:{} {}", f, ln, format_args!($str_format, $($arg)*));
        }
    };
}
