mod ch_list;
mod chat;
mod landing;
mod open_ch;
mod utils;

use crate::app::{App, View};
use tui::backend::Backend;
use tui::Frame;

pub fn draw<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let state = app.get_state();

    if !state.is_initialized() {
        landing::draw_landing(rect, app);
    }

    match state.view {
        View::ChList => {
            ch_list::draw_ch_list(rect, app);
        }
        View::OpenCh => {
            open_ch::draw_open_ch(rect, app);
        }
        View::Landing => {
            landing::draw_landing(rect, app);
        }
        View::Chat => {
            chat::draw_chat(rect, app);
        }
    }
}
