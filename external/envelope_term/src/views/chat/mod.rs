use crate::app::App;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use super::utils;

pub(crate) fn draw_chat<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    utils::check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(6),
                Constraint::Min(10),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(size);
    {
        let head_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3), //
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        let tabs = utils::draw_tabs(app.get_state());
        rect.render_widget(tabs, head_chunks[0]);

        let balance = utils::draw_balance(app.get_state());
        rect.render_widget(balance, head_chunks[1]);
    }

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(10), //
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let open_ch_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(body_chunks[0]);

    let (help_message, input, messages) =
        utils::draw_chat(app, rect, &chunks[1]);

    rect.render_widget(help_message, open_ch_chunks[1]);
    rect.render_widget(input, open_ch_chunks[2]);
    rect.render_widget(messages, open_ch_chunks[0]);

    let help = utils::draw_help(app.actions());
    rect.render_widget(help, body_chunks[1]);

    let logs = utils::draw_logs();
    rect.render_widget(logs, chunks[2]);
}
