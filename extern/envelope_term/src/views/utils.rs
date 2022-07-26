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

pub(crate) fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

pub(crate) fn draw_tabs<'a>() -> Paragraph<'a> {
    Paragraph::new(vec![Spans::from(vec![
        Span::raw("Channels "),
        Span::raw("Ch+"),
    ])])
    .style(Style::default())
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .title("Tabs")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

pub(crate) fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Envelope")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

pub(crate) fn draw_body<'a>(loading: bool, state: &AppState) -> Paragraph<'a> {
    let initialized_text = if state.is_initialized() {
        "Initialized"
    } else {
        "Not Initialized !"
    };

    let loading_text = if loading { "Loading..." } else { "" };

    let sleep_text = if let Some(sleeps) = state.count_sleep() {
        format!("Sleep count: {}", sleeps)
    } else {
        String::default()
    };

    let tick_text = if let Some(ticks) = state.count_tick() {
        format!("Tick count: {}", ticks)
    } else {
        String::default()
    };

    let sleep_text_2 = if let Some(sleeps) = state.count_sleep() {
        let mint_tc_1 = TxCandidate::new_dummy_mint_1();
        format!("Mint tx candidate: {:?}", mint_tc_1)
    } else {
        String::default()
    };

    Paragraph::new(vec![
        Spans::from(Span::raw(initialized_text)),
        Spans::from(Span::raw(loading_text)),
        Spans::from(Span::raw(sleep_text)),
        Spans::from(Span::raw(sleep_text_2)),
        Spans::from(Span::raw(tick_text)),
    ])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            // .title("Body")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

pub(crate) fn draw_duration(duration: &Duration) -> LineGauge {
    let sec = duration.as_secs();
    let label = format!("{}s", sec);
    let ratio = sec as f64 / 10.0;
    LineGauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Sleep duration"),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .line_set(line::THICK)
        .label(label)
        .ratio(ratio)
}

pub(crate) fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

pub(crate) fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(
                    Style::default().fg(Color::White).bg(Color::Black),
                )
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}

pub(crate) fn draw_dummy<'a>() -> Paragraph<'a> {
    Paragraph::new("Dummy Channel")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}
