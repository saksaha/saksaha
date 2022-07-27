use crate::app::{Actions, App, AppState, View};
use crate::io::InputMode;
use sak_types::TxCandidate;
use std::time::Duration;
use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{
    Block, BorderType, Borders, Cell, LineGauge, List, ListItem, Paragraph,
    Row, Table, Tabs,
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

pub(crate) fn draw_open_ch<'a>(app: &App) -> (Paragraph, Paragraph, List) {
    let msg = "Typing is currently disabled (helper text)";
    let style = Style::default().add_modifier(Modifier::RAPID_BLINK);

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);

    let help_message = Paragraph::new(text);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Type your friend's public key"),
        );

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();

    // let messages: Vec<ListItem> =
    //     vec![ListItem::new(vec![Spans::from(Span::raw("some message"))])];

    let messages = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Messages"));

    (help_message, input, messages)
}

pub(crate) fn draw_tabs<'a>(state: &AppState) -> Tabs {
    let labels = ["Channels", "Open channel", "Chat (#id)"]
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let tabs = Tabs::new(labels)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(0)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );

    tabs
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

pub(crate) fn draw_ch_list<'a>(
    loading: bool,
    state: &AppState,
) -> Paragraph<'a> {
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

    let foo: String = state.ch_list.join("/");

    Paragraph::new(vec![
        Spans::from(Span::raw(initialized_text)),
        Spans::from(Span::raw(loading_text)),
        Spans::from(Span::raw(sleep_text)),
        Spans::from(Span::raw(foo)),
    ])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .title("Channel list")
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

pub(crate) fn draw_help(actions: &Actions) -> Paragraph {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut v = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                action.to_string()
                // String::from("")
            };

            v.push(Span::styled(key.to_string() + " ", key_style));
            v.push(Span::styled(help, help_style));
            v.push(Span::from(" / "));
        }
    }

    Paragraph::new(Spans::from(v))
        .style(Style::default())
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Shortcuts")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
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

// pub(crate) fn __draw_help(actions: &Actions) -> Table {
//     let key_style = Style::default().fg(Color::LightCyan);
//     let help_style = Style::default().fg(Color::Gray);

//     let mut rows = vec![];
//     for action in actions.actions().iter() {
//         let mut first = true;
//         for key in action.keys() {
//             let help = if first {
//                 first = false;
//                 action.to_string()
//             } else {
//                 String::from("")
//             };
//             let row = Row::new(vec![
//                 Cell::from(Span::styled(key.to_string(), key_style)),
//                 Cell::from(Span::styled(help, help_style)),
//             ]);
//             rows.push(row);
//         }
//     }

//     Table::new(rows)
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .border_type(BorderType::Plain)
//                 .title("Help"),
//         )
//         .widths(&[Constraint::Length(11), Constraint::Min(20)])
//         .column_spacing(1)
// }
