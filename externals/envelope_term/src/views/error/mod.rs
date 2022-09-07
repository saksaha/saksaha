use super::utils;
use crate::envelope::AppState;
use tokio::sync::RwLockWriteGuard;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

pub(crate) fn draw_error<'b, B>(
    rect: &mut Frame<B>,
    // app: &Envelope,
    _state: &mut RwLockWriteGuard<'b, AppState>,
) where
    B: Backend,
{
    let size = rect.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(28)])
        .split(size);

    let error_msg = utils::draw_error();
    rect.render_widget(error_msg, chunks[0]);
}
