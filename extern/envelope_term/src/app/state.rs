use std::time::Duration;

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum View {
    Landing,
    ChList,
    OpenCh,
    Chat,
}

#[derive(Debug)]
pub(crate) struct AppState {
    initialized: bool,
    duration: Duration,
    counter_sleep: u32,
    counter_tick: u64,
    pub view: View,
}

impl AppState {
    pub fn initialized() -> Self {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;

        AppState {
            initialized: true,
            duration,
            counter_sleep,
            counter_tick,
            view: View::Landing,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn incr_sleep(&mut self) {
        // if let Self::Initialized { counter_sleep, .. } = self {
        //     *counter_sleep += 1;
        // }
        if self.initialized {
            self.counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        // if let Self::Initialized { counter_tick, .. } = self {
        //     *counter_tick += 1;
        // }

        if self.initialized {
            self.counter_tick += 1;
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if self.initialized {
            Some(self.counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if self.initialized {
            Some(self.counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if self.initialized {
            Some(&self.duration)
        } else {
            None
        }
    }

    pub fn increment_delay(&mut self) {
        if self.initialized {
            let secs = (self.duration.as_secs() + 1).clamp(1, 10);
            self.duration = Duration::from_secs(secs);
        }
    }

    pub fn decrement_delay(&mut self) {
        if self.initialized {
            let secs = (self.duration.as_secs() - 1).clamp(1, 10);
            self.duration = Duration::from_secs(secs);
        }
    }

    pub fn set_view_landing(&mut self) {
        if self.initialized {
            self.view = View::Landing;
        }
    }

    pub fn set_view_open_ch(&mut self) {
        if self.initialized {
            self.view = View::OpenCh;
        }
    }

    pub fn set_view_chat(&mut self) {
        if self.initialized {
            self.view = View::Chat;
        }
    }

    pub fn set_view_ch_list(&mut self) {
        if self.initialized {
            self.view = View::ChList;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            initialized: false,
            duration: Duration::from_secs(1),
            counter_sleep: 0,
            counter_tick: 0,
            view: View::Landing,
        }
    }
}
