use crate::inputs::events::Events;
use crate::inputs::InputEvent;
// use crate::io::handler::IoAsyncHandler;
use crate::io::InputMode;
// use crate::io::IoEvent;
use crate::views;
use crate::EnvelopeError;
use crate::{AppReturn, Envelope};
use std::io::Stdout;
use std::sync::Arc;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub(super) struct UIRoutine;

impl UIRoutine {
    pub async fn run(&self, envelope: Arc<Envelope>) -> Result<(), EnvelopeError> {
        let mut terminal = configure_terminal()?;

        let tick_rate = Duration::from_millis(1000);
        let mut events = Events::new(tick_rate);

        let envelope = envelope.clone();
        // envelope.dispatch(IoEvent::Initialize).await;
        envelope
            .dispatch(crate::envelope::Action::Initialize)
            .await?;

        loop {
            let mut state = envelope.get_state().write().await;

            terminal.draw(|rect| views::draw(rect, &mut state))?;

            let result = match events.next().await {
                InputEvent::Input(key) => envelope.handle_key_input(key, state).await,
                // InputEvent::Input(key) => match state.input_mode {
                //     InputMode::Normal => {
                //         envelope.handle_normal_key(key, state).await
                //     }
                //     InputMode::Editing => {
                //         envelope.handle_edit_key(key, state).await
                //     }
                // },
                InputEvent::Tick => envelope.update_on_tick().await,
            };

            if result == AppReturn::Exit {
                events.close();
                break;
            }
        }

        terminal.clear()?;
        terminal.show_cursor()?;
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }
}

fn configure_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, EnvelopeError> {
    let stdout = std::io::stdout();

    crossterm::terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    Ok(terminal)
}
