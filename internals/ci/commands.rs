use std::{
    sync::Once,
};
use clap::App;

static mut T: *const Vec<Command> = 0 as *const Vec<Command>;
static mut T2: *const Vec<Box<dyn FnOnce()>> = 0 as *const Vec<Box<dyn FnOnce()>>;

static ONCE: Once = Once::new();

struct Command<'a, 'b> {
    def: Box<dyn FnOnce(App<'a, 'b>) -> App<'a, 'b>>,
    exec: Box<dyn FnOnce() -> ()>,
    t: Box<dyn FnOnce()>,
}

fn dev<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    print!("dev\n");
    return app;
}

fn dev_exec() {

}

pub fn get_commands() {
    ONCE.call_once(|| {
        let commands = vec!(
            Command {
                def: Box::new(dev),
                exec: Box::new(dev_exec),
                t: Box::new(move || println!("OK")),
            },
        );

        unsafe {
            T = &commands;
        }
    });

    let f: Box<dyn Fn()> = Box::new(|| println!("OK\n"));
    let b = &*f;
    b();
    // b();
    // (*b)();

    unsafe {
        for e in &*T2 {
            // let f = *e;
            // // std::mem::transmute::<*const (), fn() -> ()>(*e);
            // f();
            // (a.t)();
            // let b = &*i.t;
            // b();
            // b();
            // b();
            // b(vec!(1));
            // let b = *i.t;
            // b();
            // i.exec();
        }
    }
}
