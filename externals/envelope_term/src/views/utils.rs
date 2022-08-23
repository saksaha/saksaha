use crate::envelope::{Actions, AppState, Envelope};
use crate::io::InputMode;
use tui::backend::Backend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{
    Block, BorderType, Borders, LineGauge, List, ListItem, Paragraph, Tabs,
};
use tui::Frame;
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};
use unicode_width::UnicodeWidthStr;

pub(crate) fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

pub(crate) fn draw_open_ch<'a, B>(
    app: &'a Envelope,
    rect: &mut Frame<B>,
    chunks: &Vec<Rect>,
) -> (Paragraph<'a>, Paragraph<'a>, List<'a>)
where
    B: Backend,
{
    let state = app.get_state();

    let (msg, style) = match app.get_state().input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "q",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to exit, "),
                Span::styled(
                    "i",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "Esc",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to stop editing, "),
                Span::styled(
                    "Enter",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };

    let help_msg = {
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        Paragraph::new(text)
    };

    let input = Paragraph::new(state.input_text.as_ref())
        .style(match state.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Type your friend's public key"),
        );

    let input_returned = {
        let content = vec![Spans::from(Span::raw(format!(
            "Her pk: {}",
            state.input_returned
        )))];

        let v = vec![ListItem::new(content)];

        List::new(v).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Open channel progress"),
        )
    };

    match state.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            rect.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + state.input_text.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 2,
            )
        }
    }

    (help_msg, input, input_returned)
}

pub(crate) fn draw_tabs<'a>(state: &'a AppState) -> Tabs {
    let labels = ["Channels", "Open channel", "Chat (#)"]
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(3);
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

pub(crate) fn draw_balance<'a>(state: &'a AppState) -> Paragraph {
    // let (msg, style) = match state.{
    let (msg, style) = (
        vec![
            Span::styled("My Balance: ", Style::default()),
            Span::styled(
                format!("     {} ", state.balance),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightYellow),
            ),
            Span::styled("(Sak)", Style::default()),
        ],
        Style::default(),
    );

    let balance = {
        let mut text = Text::from(Spans::from(msg));

        text.patch_style(style);

        Paragraph::new(text)
            .style(Style::default())
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .title("Balance")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray))
                    .border_type(BorderType::Plain),
            )
    };

    balance
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

pub(crate) fn draw_ch_list<'a>(state: &AppState) -> List<'a> {
    let items: Vec<ListItem> = state
        .ch_list
        .iter()
        .enumerate()
        .map(|(idx, i)| {
            let ch = format!("{}. {}", idx, i.channel.ch_id.to_owned());
            ListItem::new(ch)
                .style(Style::default().fg(Color::White).bg(Color::Black))
        })
        .collect();

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
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
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
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

pub(crate) fn draw_chat<'a, B>(
    app: &'a Envelope,
    rect: &mut Frame<B>,
    chunks: &Rect,
) -> (Paragraph<'a>, Paragraph<'a>, Paragraph<'a>)
where
    B: Backend,
{
    let state = app.get_state();

    let (msg, style) = match app.get_state().input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "q",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to exit, "),
                Span::styled(
                    "i",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "Esc",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to stop editing, "),
                Span::styled(
                    "Enter",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };

    let help_msg = {
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        Paragraph::new(text)
    };

    let input = Paragraph::new(state.input_text.as_ref())
        .style(match state.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Write a message"),
        );

    let message_panel = {
        let content: Vec<Spans> = state
            .chats
            .iter()
            .rev()
            .map(|m| {
                let date = m.date.to_string();

                Spans::from(vec![
                    Span::styled(date, Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("({}) ", m.user),
                        Style::default().fg(Color::LightYellow),
                    ),
                    Span::raw(format!("{}", m.msg)),
                ])
            })
            .collect();

        Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                "Messages",
                Style::default().add_modifier(Modifier::BOLD),
            )))
            .scroll((state.scroll_messages_view() as u16, 0))
    };

    match state.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            rect.set_cursor(
                // Put cursor past the end of the input text
                chunks.x + state.input_text.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks.height,
            )
        }
    }

    (help_msg, input, message_panel)
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
