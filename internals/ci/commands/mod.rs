mod build;
mod dev;
mod clean;
mod expand;

use clap::{App, ArgMatches};
use once_cell::sync::Lazy;
use std::{
    sync::Mutex,
};

pub static COMMANDS: Lazy<Mutex<Vec<Command>>> = Lazy::new(|| {
    let v = vec![
        Command {
            def: Box::new(build::build_command),
            exec: Box::new(build::build_exec),
        },
        Command {
            def: Box::new(dev::dev_command),
            exec: Box::new(dev::dev_exec),
        },
        Command {
            def: Box::new(clean::clean_command),
            exec: Box::new(clean::clean_exec),
        },
        Command {
            def: Box::new(expand::expand_command),
            exec: Box::new(expand::expand_exec),
        },
    ];
    Mutex::new(v)
});

pub struct Command<'a, 'b> {
    pub def: Box<dyn Fn(App<'a, 'b>) -> App<'a, 'b> + Send>,
    pub exec: Box<dyn Fn(&ArgMatches) -> () + Send>,
}
