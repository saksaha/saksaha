#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn add_one(x: i32) -> i32 {
    return x + 1;
}

#[macro_export]
macro_rules! log {
    (add $($addend:expr),+) => {
        let mut sum = 0;
        $(
            sum += $addend;
         )*

        println!("Sum: {}", sum);
    }
}

// #[macro_export]
// macro_rules! log1 {
//     ($($arg: expr)*) => {{
//         println!("Sum: {} | Product: {}", sum, product);
//     }}
// }
