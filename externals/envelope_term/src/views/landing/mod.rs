use crate::envelope::{Actions, AppState, Envelope, View};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::{symbols, Frame};

use super::utils;

pub(crate) fn draw_landing<B>(rect: &mut Frame<B>, app: &Envelope)
where
    B: Backend,
{
    let size = rect.size();
    utils::check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(4),
                Constraint::Min(10),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(size);

    let title = utils::draw_title();
    rect.render_widget(title, chunks[0]);

    let logs = utils::draw_logs();
    rect.render_widget(logs, chunks[1]);
}
