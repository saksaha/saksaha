pub struct Routine {
    is_running: bool,
}

impl Routine {
    pub fn new() -> Routine {
        Routine { is_running: false, }
    }

    pub fn run(&self) {

    }

    pub async fn wakeup(&self) {

    }
}
