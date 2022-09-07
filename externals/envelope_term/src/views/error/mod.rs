use super::utils;
use crate::envelope::{AppState, Envelope, View};
use tokio::sync::RwLockWriteGuard;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::{symbols, Frame};

pub(crate) fn draw_error<'b, B>(
    rect: &mut Frame<B>,
    // app: &Envelope,
    _state: &mut RwLockWriteGuard<'b, AppState>,
) where
    B: Backend,
{
    let size = rect.size();
    utils::check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3)].as_ref())
        .split(size);

    let title = utils::draw_error();
    rect.render_widget(title, chunks[0]);
}
