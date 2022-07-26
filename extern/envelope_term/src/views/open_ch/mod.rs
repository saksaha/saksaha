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
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(size);

    let tabs = utils::draw_tabs(app.get_state());
    rect.render_widget(tabs, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(4)].as_ref())
        .split(chunks[1]);

    let open_ch_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(body_chunks[0]);

    let (help_message, input, messages) = utils::draw_open_ch(app.get_state());
    rect.render_widget(help_message, open_ch_chunks[0]);
    rect.render_widget(input, open_ch_chunks[1]);
    rect.render_widget(messages, open_ch_chunks[2]);

    let help = utils::draw_help(app.actions());
    rect.render_widget(help, body_chunks[1]);

    let logs = utils::draw_logs();
    rect.render_widget(logs, chunks[2]);
}
