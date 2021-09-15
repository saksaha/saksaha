use clap::App;
use once_cell::sync::Lazy;
use std::{
    sync::{Mutex},
};

static COMMANDS: Lazy<Mutex<Vec<Command>>> = Lazy::new(|| {
    let v = vec![
        Command {
            def: Box::new(dev),
            exec: Box::new(dev_exec),
        },
        Command {
            def: Box::new(dev),
            exec: Box::new(dev_exec),
        },
    ];
    Mutex::new(v)
});

struct Command<'a, 'b> {
    def: Box<dyn Fn(App<'a, 'b>) -> App<'a, 'b> + Send>,
    exec: Box<dyn Fn() -> () + Send>,
}

fn dev<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    print!("dev\n");
    return app;
}

fn dev_exec() {
    print!("dev exec\n");
}

pub fn t() {
    let a = COMMANDS.lock().unwrap();
    for x in a.iter() {
        let b = &x.exec;
        b();
    }
}
