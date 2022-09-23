mod ch_list;
mod chat;
mod error;
mod landing;
mod open_ch;
mod utils;

use crate::envelope::{AppState, View};
use tokio::sync::RwLockWriteGuard;
use tui::backend::Backend;
use tui::Frame;

use self::utils::check_size;

pub(crate) fn draw<'a, 'b, B>(rect: &mut Frame<'a, B>, state: &mut RwLockWriteGuard<'b, AppState>)
where
    B: Backend,
{
    if !check_size(&rect.size()) {
        state.view = View::Error;
    } else {
        if state.view == View::Error {
            state.view = View::ChList;
        }
    }

    match state.view {
        View::ChList => {
            ch_list::draw_ch_list(rect, state);
        }
        View::OpenCh => {
            open_ch::draw_open_ch(rect, state);
        }
        View::Landing => {
            landing::draw_landing(rect, state);
        }
        View::Chat => {
            chat::draw_chat(rect, state);
        }
        View::Error => {
            error::draw_error(rect, state);
        }
    }
}
