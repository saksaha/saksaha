use tui::widgets::ListState;

use crate::io::InputMode;
use chrono::{DateTime, Local};

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum View {
    Landing,
    ChList,
    OpenCh,
    Chat,
}

#[derive(Debug)]
pub struct AppState {
    initialized: bool,
    counter_sleep: u32,
    counter_tick: u64,
    scroll_messages_view: usize,
    pub is_loading: bool,
    pub ch_list_state: ListState,
    pub ch_list: Vec<String>,
    pub input_mode: InputMode,
    pub input_text: String,
    pub input_returned: String,
    pub chat_input: String,
    pub chats: Vec<ChatMessage>,
    pub view: View,
}

impl AppState {
    pub fn initialized() -> Self {
        let counter_sleep = 0;
        let counter_tick = 0;

        AppState {
            initialized: true,
            counter_sleep,
            counter_tick,
            scroll_messages_view: 0,
            ch_list_state: ListState::default(),
            ch_list: vec![],
            is_loading: false,
            input_mode: InputMode::Normal,
            input_text: String::default(),
            input_returned: String::default(),
            chat_input: String::default(),
            chats: vec![],
            view: View::Landing,
        }
    }
    pub fn scroll_messages_view(&self) -> usize {
        self.scroll_messages_view
    }

    pub fn messages_scroll(&mut self, movement: ScrollMovement) {
        match movement {
            ScrollMovement::Up => {
                if self.scroll_messages_view > 0 {
                    self.scroll_messages_view -= 1;
                }
            }
            ScrollMovement::Down => {
                self.scroll_messages_view += 1;
            }
            ScrollMovement::Start => {
                self.scroll_messages_view += 0;
            }
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn incr_sleep(&mut self) {
        if self.initialized {
            self.counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if self.initialized {
            self.counter_tick += 1;
        }
    }

    pub fn set_ch_list_state(&mut self, data: String) {
        self.ch_list = match serde_json::from_str(&data) {
            Ok(c) => c,
            Err(err) => {
                panic!("Cannot Deserialize `data`:, err: {}", err);
            }
        };
    }

    pub fn set_msg_state(&mut self, data: String) {
        self.chats = match serde_json::from_str::<Vec<String>>(&data) {
            Ok(c) => c.into_iter().map(|m| ChatMessage::new(m)).collect(),
            Err(err) => {
                panic!("Cannot Deserialize `msg`:, err: {}", err);
            }
        };
    }

    pub fn set_input_messages(&mut self, msg: String) {
        self.chats.push(ChatMessage::new(msg));
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

    pub fn next_ch(&mut self) {
        let i = match self.ch_list_state.selected() {
            Some(i) => {
                if i >= self.ch_list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.ch_list_state.select(Some(i));
    }

    pub fn previous_ch(&mut self) {
        let i = match self.ch_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.ch_list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.ch_list_state.select(Some(i));
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            initialized: false,
            counter_sleep: 0,
            counter_tick: 0,
            scroll_messages_view: 0,
            ch_list_state: ListState::default(),
            ch_list: vec![],
            is_loading: false,
            input_mode: InputMode::Normal,
            input_text: String::default(),
            input_returned: String::default(),
            chat_input: String::default(),
            chats: vec![],
            view: View::Landing,
        }
    }
}

#[derive(Debug)]
pub struct ChatMessage {
    pub date: DateTime<Local>,
    pub msg: String,
}

impl ChatMessage {
    pub fn new(msg: String) -> ChatMessage {
        ChatMessage {
            date: Local::now(),
            msg,
        }
    }
}

#[derive(Debug)]
pub struct ChannelState {
    pub channel_name: String,
    pub her_pk: String,
    pub my_pk: String,
}

impl ChannelState {
    pub fn new(
        channel_name: String,
        her_pk: String,
        my_pk: String,
    ) -> ChannelState {
        ChannelState {
            channel_name,
            her_pk,
            my_pk,
        }
    }
}

pub enum ScrollMovement {
    Up,
    Down,
    Start,
}
