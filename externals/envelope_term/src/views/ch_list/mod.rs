use super::utils;
use crate::envelope::{AppState, Envelope};
use tokio::sync::{OwnedRwLockWriteGuard, RwLockReadGuard, RwLockWriteGuard};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

pub(crate) fn draw_ch_list<'a, 'b, B>(
    rect: &mut Frame<'a, B>,
    // envelope: &Envelope,
    state: &mut RwLockWriteGuard<'b, AppState>,
) where
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

        // let state = envelope.get_state().read().await;
        let tabs = utils::draw_tabs(&state);
        rect.render_widget(tabs, head_chunks[0]);

        let balance = utils::draw_balance(&state);
        rect.render_widget(balance, head_chunks[1]);
    }

    {
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

        let ch_list = utils::draw_ch_list(&state);
        rect.render_stateful_widget(
            ch_list,
            body_chunks[0],
            &mut state.ch_list_state,
        );

        // let help = utils::draw_help(envelope.get_actions());
        // rect.render_widget(help, body_chunks[1]);
    }

    let logs = utils::draw_logs();
    rect.render_widget(logs, chunks[2]);
}
