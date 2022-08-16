#![feature(test)]
extern crate test;

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
            app.render(terminal);
        }
        res = res
            .and(app.react_to_event(event::read()?))
            .and(app.react_to_code_runner());

        app.update();
    }
    Ok(())
}

#[cfg(test)]
#[allow(soft_unstable)]
mod tests {
    use crate::structure::model::Model;
    use crate::structure::settings::Settings;
    use crate::structure::view::View;
    use test::Bencher;

    #[bench]
    fn bench_table(b: &mut Bencher) {
        let model = Model::new_ref(Settings::default());
        let view = View::from(&model);

        b.iter(|| {
            // Inner closure, the actual test
            view.get_problems_to_select(30);
        });
    }

    #[bench]
    fn bench_detailed_problem(b: &mut Bencher) {
        let model = Model::new_ref(Settings::default());
        let view = View::from(&model);

        b.iter(|| {
            // Inner closure, the actual test
            view.detailed_problem();
        });
    }
}
