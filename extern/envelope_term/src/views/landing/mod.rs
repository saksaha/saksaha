use crate::app::{Actions, App, AppState, View};
use sak_types::TxCandidate;
use std::time::Duration;
use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{
    Block, BorderType, Borders, Cell, LineGauge, Paragraph, Row, Table,
};
use tui::{symbols, Frame};
use tui_logger::TuiLoggerWidget;

use super::utils;

pub(crate) fn draw_landing<B>(rect: &mut Frame<B>, app: &App)
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

    let title = utils::draw_title();
    rect.render_widget(title, chunks[0]);

    let logs = utils::draw_logs();
    rect.render_widget(logs, chunks[1]);
}
