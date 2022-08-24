mod ch_list;
mod chat;
mod landing;
mod open_ch;
mod utils;

use crate::envelope::{AppState, Envelope, View};
use tokio::sync::{OwnedRwLockWriteGuard, RwLockReadGuard, RwLockWriteGuard};
use tui::backend::Backend;
use tui::Frame;

pub(crate) fn draw<'a, 'b, B>(
    rect: &mut Frame<'a, B>,
    state: &mut RwLockWriteGuard<'b, AppState>,
) where
    B: Backend,
{
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
    }
}
