#![feature(test)]
use crate::structure::ui::UIElement;
extern crate test;

use test::Bencher;
use crate::structure::controller::AfterEvent;
use clap;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod arguments;
mod complexity;
mod interface;
mod structure;

use arguments::AppArgs;
use structure::AppState;

fn main() {
    let args = AppArgs::parse();
    let app = AppState::from(args);

    // Setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let result = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    terminal.show_cursor().unwrap();

    result.unwrap();
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: AppState) -> io::Result<()> {
    let mut res = AfterEvent::DoRefresh;
    while !res.is_quit() {
        if let AfterEvent::DoRefresh = res {
            terminal.draw(|frame| app.render(frame))?;
        }

        res = app.react_to_event(event::read()?);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::structure::ui::UIElement;
    use crate::AppState;
    use test::{Bencher, black_box};

    #[bench]
    fn setup_available_problems(b: &mut Bencher) {
        let app = AppState::default();

        b.iter(|| {
            crate::structure::ui::AvailableProblems::setup(&app.view)
        })
    }

    #[bench]
    fn setup_problem_preview(b: &mut Bencher) {
        let app = AppState::default();
        b.iter(|| {
            crate::structure::ui::ProblemView::setup(&app.view)
        })
    }
}