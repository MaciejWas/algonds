use crate::application::controller::AfterEvent::*;
use std::time::Duration;
use std::io;
use clap::Parser;
use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod application;
mod arguments;
mod interface;
mod data;

use application::AppState;
use arguments::AppArgs;

const EVENT_CHECK_DUR: Duration = Duration::from_millis(300);

fn main() {
    let args = AppArgs::parse();
    let mut app = AppState::from(args);

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let result = run_app(&mut terminal, &mut app);
    app.controller.kill_all_processes();

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen,).unwrap();
    terminal.show_cursor().unwrap();

    result.unwrap();
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Result<()> {
    let mut action = DoRefresh;
    while action != Quit {
        if action == DoRefresh {
            terminal.draw(|frame| app.render(frame))?;
        }

        let there_is_a_new_event = event::poll(EVENT_CHECK_DUR)?;
        action = if there_is_a_new_event {
            app.react_to_event(event::read()?)
        } else {
            app.react_to_code_runner()
        };
    }
    Ok(())
}
