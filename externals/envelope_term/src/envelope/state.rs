use crate::{io::InputMode, wallet_sdk, EnvelopeError};
use core::fmt;
use envelope_contract::{Channel, ChatMessage};
use log::{info, warn};
use tui::widgets::ListState;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum View {
    Landing,
    ChList,
    OpenCh,
    Chat,
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            View::Landing => write!(f, "landing"),
            View::ChList => write!(f, "Channels [1]"),
            View::OpenCh => write!(f, "Open channel [2]"),
            View::Chat => write!(f, "Chat [3]"),
        }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub is_initialized: bool,
    pub scroll_messages_view: usize,
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
    pub selected_ch: Channel,
    pub image_count: u16,
}

impl AppState {
    pub fn initialized() -> Self {
        AppState {
            is_initialized: false,
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
            selected_ch: Channel::default(),
            image_count: 0,
        }
    }

    pub fn scroll_messages_view(&self) -> usize {
        self.scroll_messages_view
    }

    // pub fn get_is_initialized(&self) -> bool {
    //     self.is_initialized
    // }

    // pub fn set_is_initialized(&mut self, is_initialized: bool) {
    //     self.is_initialized = is_initialized;
    //     self.view = View::ChList;
    // }

    // pub fn set_ch_list(
    //     &mut self,
    //     new_ch: ChannelState,
    // ) -> Result<(), EnvelopeError> {
    //     if !self.ch_list.contains(&new_ch) {
    //         self.ch_list.push(new_ch);
    //     }
    //     Ok(())
    // }

    // pub fn set_chats(&mut self, data: Vec<ChatMessage>, my_pk: String) {
    //     self.chats = data;
    // }

    // pub fn set_input_messages(&mut self, msg: String) {
    //     let user = String::from("me");

    //     self.chats.push(ChatMessage::new(msg, user));
    // }

    // pub fn set_view_landing(&mut self) {
    //     if self.is_initialized {
    //         self.view = View::Landing;
    //     }
    // }

    // pub fn set_view_open_ch(&mut self) {
    //     if self.is_initialized {
    //         self.view = View::OpenCh;
    //     }
    // }

    pub fn set_view_chat(&mut self) {
        if self.is_initialized {
            self.view = View::Chat;
        }
    }

    // pub fn set_view_ch_list(&mut self) {
    //     if self.is_initialized {
    //         self.view = View::ChList;
    //     }
    // }

    pub async fn set_balance(
        &mut self,
        wallet_endpoint: String,
        user_pk: String,
    ) {
        let my_str = String::from("");

        println!("{}", my_str.clone());
        // TODO get user_id via params
        // let tmp_user_id = USER_1.to_owned();
        // let tmp_user_id = "".to_owned();
        // let balance = match get_balance_from_wallet(&tmp_user_id).await {

        {
            // update the coin_manager in wallet
            let _ =
                wallet_sdk::update_wallet(wallet_endpoint.clone(), &user_pk)
                    .await;
        }

        let balance = match wallet_sdk::get_balance_from_wallet(
            wallet_endpoint,
            &user_pk,
        )
        .await
        {
            Ok(resp) => {
                info!("Success to get response from wallet");

                let result = match resp.result {
                    Some(b) => {
                        info!("Updating balance, balance: {:?}", b.balance.val);

                        b.balance.val.to_string()
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

    // pub fn next_ch(&mut self) {
    //     let i = match self.ch_list_state.selected() {
    //         Some(i) => {
    //             if i >= self.ch_list.len() - 1 {
    //                 0
    //             } else {
    //                 i + 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.ch_list_state.select(Some(i));
    // }

    // pub fn previous_ch(&mut self) {
    //     let i = match self.ch_list_state.selected() {
    //         Some(i) => {
    //             if i == 0 {
    //                 self.ch_list.len() - 1
    //             } else {
    //                 i - 1
    //             }
    //         }
    //         None => 0,
    //     };
    //     self.ch_list_state.select(Some(i));
    // }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            is_initialized: false,
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
            selected_ch: Channel::default(),
            image_count: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChannelState {
    pub channel: Channel,
}

impl ChannelState {
    pub fn new(channel: Channel) -> ChannelState {
        ChannelState { channel }
    }
}

pub enum ScrollMovement {
    Up,
    Down,
    Start,
}
