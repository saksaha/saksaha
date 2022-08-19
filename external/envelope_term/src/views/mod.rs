mod ch_list;
mod chat;
mod landing;
mod open_ch;
mod utils;

use crate::envelope::{Envelope, View};
use tui::backend::Backend;
use tui::Frame;

pub fn draw<B>(rect: &mut Frame<B>, envelope: &mut Envelope)
where
    B: Backend,
{
    let state = envelope.get_state();

    if !state.is_initialized() {
        landing::draw_landing(rect, envelope);
    }

    match state.view {
        View::ChList => {
            ch_list::draw_ch_list(rect, envelope);
        }
        View::OpenCh => {
            open_ch::draw_open_ch(rect, envelope);
        }
        View::Landing => {
            landing::draw_landing(rect, envelope);
        }
        View::Chat => {
            chat::draw_chat(rect, envelope);
        }
    }
}
