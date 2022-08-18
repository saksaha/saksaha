use envelope_types::{Channel, ChatMessage};
use tui::widgets::ListState;

use crate::db::USER_1;
use crate::EnvelopeError;
use crate::{io::InputMode, term::get_balance_from_wallet};
use log::{info, warn};

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
    pub ch_list: Vec<ChannelState>,
    pub input_mode: InputMode,
    pub input_text: String,
    pub input_returned: String,
    pub chat_input: String,
    pub chats: Vec<ChatMessage>,
    pub view: View,
    pub balance: String,
    pub selected_ch_id: String,
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
            balance: String::from("0"),
            selected_ch_id: String::default(),
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

    pub fn set_ch_list(
        &mut self,
        new_ch: ChannelState,
    ) -> Result<(), EnvelopeError> {
        if !self.ch_list.contains(&new_ch) {
            self.ch_list.push(new_ch);
        }
        Ok(())
    }

    pub fn set_chats(&mut self, data: Vec<ChatMessage>, my_pk: String) {
        self.chats = data;
    }

    // pub fn set_input_messages(&mut self, msg: String) {
    //     let user = String::from("me");

    //     self.chats.push(ChatMessage::new(msg, user));
    // }

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

    pub async fn set_balance(&mut self) {
        //TODO get user_id via params
        let tmp_user_id = USER_1.to_owned();

        let balance = match get_balance_from_wallet(&tmp_user_id).await {
            Ok(resp) => {
                info!("Success to get response from wallet");
                let result = match resp.result {
                    Some(b) => {
                        info!("Updating balance, balance: {}", b);
                        b
                    }
                    None => {
                        warn!("Failed to get balance, Set balance as default value \'0\'");
                        String::from("0")
                    }
                };

                result
            }

            Err(err) => {
                warn!("Failed to get balance from wallet, err: {}", err);

                String::from("0")
            }
        };

        self.balance = balance;
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
            balance: String::from("0"),
            selected_ch_id: String::default(),
        }
    }
}

// #[derive(Debug)]
// pub struct ChatMessage {
// pub date: DateTime<Local>,
//     pub msg: String,
//     pub user: String,
// }

// impl ChatMessage {
//     pub fn new(msg: String, user: String) -> ChatMessage {
//         ChatMessage {
//             date: Local::now(),
//             msg,
//             user,
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub struct ChannelState {
    pub channel: Channel,
    pub her_pk: String,
}

impl ChannelState {
    pub fn new(channel: Channel, her_pk: String) -> ChannelState {
        ChannelState { channel, her_pk }
    }
}

pub enum ScrollMovement {
    Up,
    Down,
    Start,
}
