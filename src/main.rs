#![feature(test)]
use crate::AfterEvent::Quit;
use std::fs::File;
use crate::application::ui::UIElement;
use crate::AfterEvent::DoRefresh;
use std::time::Duration;
extern crate test;
use crate::application::controller::AfterEvent;
use clap;
use clap::Parser;
use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod application;
mod arguments;
mod complexity;
mod interface;

use application::AppState;
use arguments::AppArgs;

const EVENT_CHECK_DUR: Duration = Duration::from_millis(300);
const LOG_FILE: &str = "./algonds.log";

fn main() {
    let args = AppArgs::parse();
    let app = AppState::from(args);

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let result = run_app(&mut terminal, app);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    ).unwrap();
    terminal.show_cursor().unwrap();

    result.unwrap();
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: AppState) -> io::Result<()> {
    let mut action = DoRefresh;
    while action != Quit {
        if action == DoRefresh {
            terminal.draw(|frame| app.render(frame))?;
        }

        let there_is_a_new_event = event::poll(EVENT_CHECK_DUR).unwrap_or(false);
        let new_action = if there_is_a_new_event {
            app.react_to_event(event::read()?)
        } else {
            app.react_to_code_runner()
        };

        action = new_action.or(action);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::application::ui::UIElement;
    use crate::AppState;
    use test::Bencher;

    #[bench]
    fn setup_available_problems(b: &mut Bencher) {
        let app = AppState::default();

        b.iter(|| crate::application::ui::AvailableProblems::setup(&app.view))
    }

    #[bench]
    fn setup_problem_preview(b: &mut Bencher) {
        let app = AppState::default();
        b.iter(|| crate::application::ui::ProblemView::setup(&app.view))
    }
}
