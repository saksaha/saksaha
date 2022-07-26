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

pub(crate) fn draw_open_ch<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    utils::check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(size);

    let title = utils::draw_title();
    rect.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20)].as_ref())
        .split(chunks[0]);

    let body = utils::draw_dummy();
    rect.render_widget(body, body_chunks[0]);
}
