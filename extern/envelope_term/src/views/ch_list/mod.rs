use super::utils;
use crate::app::{Actions, App, AppState, View};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::{symbols, Frame};

pub(crate) fn draw_ch_list<B>(rect: &mut Frame<B>, app: &mut App)
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
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(size);

    let tabs = utils::draw_tabs(app.get_state());
    rect.render_widget(tabs, chunks[0]);

    {
        let body_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(4)].as_ref())
            .split(chunks[1]);

        let ch_list = utils::draw_ch_list(app.is_loading(), app.get_state());
        // rect.render_widget(ch_list, body_chunks[0]);
        rect.render_stateful_widget(
            ch_list,
            body_chunks[0],
            &mut app.state.ch_list_state,
        );

        let help = utils::draw_help(app.actions());
        rect.render_widget(help, body_chunks[1]);
    }

    let logs = utils::draw_logs();
    rect.render_widget(logs, chunks[2]);
}
