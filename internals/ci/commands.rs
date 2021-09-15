use crate::log;
use clap::{App, Arg, ArgMatches, SubCommand};
use once_cell::sync::Lazy;
use std::{process::{Command as StdCommand, Stdio}, sync::Mutex};

pub static COMMANDS: Lazy<Mutex<Vec<Command>>> = Lazy::new(|| {
    let v = vec![
        Command {
            def: Box::new(dev),
            exec: Box::new(dev_exec),
        },
        // Command {
        //     def: Box::new(dev),
        //     exec: Box::new(dev_exec),
        // },
    ];
    Mutex::new(v)
});

pub struct Command<'a, 'b> {
    pub def: Box<dyn Fn(App<'a, 'b>) -> App<'a, 'b> + Send>,
    pub exec: Box<dyn Fn(&ArgMatches) -> () + Send>,
}

fn get_current_dir() -> String {
    let curr_dir = match std::env::current_dir() {
        Ok(c) => {
            let p = c
                .into_os_string()
                .into_string()
                .expect("Current directory needs to be retrieved");
            p
        }
        Err(err) => {
            log!("Cannot retreive current directory, err: {}\n", err);
            String::from("")
        }
    };
    curr_dir
}

fn dev<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name("dev")
            .version("0.1")
            .arg(Arg::with_name("args").multiple(true)),
    )
}

fn dev_exec(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("dev") {
        let program = "cargo";

        let args = match matches.values_of("args") {
            Some(a) => {
                let args: Vec<_> = a.collect();
                format!("build {}", args.join(" "))
            }
            None => String::from("build"),
        };

        let curr_dir = get_current_dir();

        log!("Executing `{} {}`, curr_dir: {:?}\n", program, args, curr_dir);

        StdCommand::new(program)
            .arg(args)
            .stdout(Stdio::inherit())
            .output()
            .expect("failed to run");
    }
}
