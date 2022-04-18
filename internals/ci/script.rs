use clap::{ArgMatches, Command};

// pub(crate) trait Scriptify2 {
//     fn name(&self) -> &str;

//     fn define<'a, 'b>(&self, app: Command<'a>) -> Command<'a>;

//     fn handle_matches(&self, matches: &ArgMatches) -> Option<bool>;
// }

pub(crate) trait Script2 {
    fn name() -> &'static str;

    fn define(app: Command<'static>) -> Command<'static>;

    fn handle_matches(matches: &ArgMatches) -> Option<bool>;
}

// #[macro_export]
// macro_rules! foo {
//     ($str_format: expr) => {
//         #[cfg(debug_assertions)]
//         {
//             // std::println!("[ci] {}", std::format_args!($str_format));
//         }
//     };

//     ($str_format: expr, $($arg:tt)*) => {
//         #[cfg(debug_assertions)]
//         {
//             // std::println!("[ci] {}", format_args!($str_format, $($arg)*));
//         }
//     };
// }
