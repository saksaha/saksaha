use super::Commandify;
use crate::log;
use clap::{Arg, ArgMatches, Command};

pub struct Clean;

impl Commandify for Clean {
    fn name(&self) -> &str {
        "clean"
    }

    fn def<'a, 'b>(&self, app: Command<'a>) -> Command<'a> {
        app.subcommand(
            Command::with_name(self.name())
                .arg(Arg::with_name("args").multiple(true)),
        )
    }

    fn exec(&self, matches: &ArgMatches) -> Option<bool> {
        if let Some(matches) = matches.subcommand_matches(self.name()) {
            let program = "cargo";
            let args = match matches.values_of("args") {
                Some(a) => a.collect(),
                None => vec![],
            };
            let args = [vec!["clean", "--"], args].concat();

            log!("Executing `{} {:?}`", program, args,);

            let cmd = Command::new(program)
                .args(args)
                .spawn()
                .expect("failed to run");

            cmd.wait_with_output().unwrap();

            return Some(true);
        }
        None
    }
}
