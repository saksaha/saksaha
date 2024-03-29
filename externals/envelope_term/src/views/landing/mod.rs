use super::utils;
use crate::envelope::{AppState, Envelope, View};
use tokio::sync::RwLockWriteGuard;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::{symbols, Frame};

pub(crate) fn draw_landing<'b, B>(
    rect: &mut Frame<B>,
    // app: &Envelope,
    _state: &mut RwLockWriteGuard<'b, AppState>,
) where
    B: Backend,
{
    let size = rect.size();
    // utils::check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(10)].as_ref())
        .split(size);

    let title = utils::draw_title();
    rect.render_widget(title, chunks[0]);

    let logs = utils::draw_logs();
    rect.render_widget(logs, chunks[1]);
}
