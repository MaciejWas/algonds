use crate::structure::common::Example;
use crate::structure::Difficulty;
use crate::structure::ModelRef;
use crate::structure::Problem;
use crate::structure::Settings;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Paragraph;
use tui::widgets::Wrap;

struct ProblemCacher {
    range: Cell<(usize, usize)>,
    problems: RefCell<Vec<Rc<Problem>>>,
}

impl Default for ProblemCacher {
    fn default() -> Self {
        Self {
            range: Cell::new((0, 0)),
            problems: RefCell::new(Vec::new()),
        }
    }
}

impl ProblemCacher {
    pub fn remember(&self, input: (usize, usize), result: &Vec<Rc<Problem>>) {
        self.range.set(input);
        let mut x = self.problems.borrow_mut();
        *x = result.clone();
    }

    pub fn get_cached(&self, input: (usize, usize)) -> Option<Vec<Rc<Problem>>> {
        if input == self.range.get() {
            return Some(self.problems.borrow().clone());
        }
        None
    }
}

pub struct View {
    model: ModelRef,
    settings: Settings,
    problem_cacher: ProblemCacher,
}

impl From<&ModelRef> for View {
    fn from(model: &ModelRef) -> Self {
        Self {
            model: Rc::clone(model),
            settings: model.settings.borrow().clone(),
            problem_cacher: ProblemCacher::default(),
        }
    }
}

impl View {
    pub fn block_with_title<'a>(&self, title: String) -> Block<'a> {
        Block::default()
            .borders(tui::widgets::Borders::ALL)
            .title(title)
            .title_style(Style::default().add_modifier(tui::style::Modifier::BOLD))
    }
    pub fn get_help(&self) -> Paragraph {
        let spans = vec![
            Spans::from(Self::bold("General".to_string())),
            Spans::from("  h - open help"),
            Spans::from("  q - quit current menu"),
            Spans::from("  ctrl + c - exit application"),
            Spans::from(""),
            Spans::from(Self::bold("Select Problem Menu".to_string())),
            Spans::from("  j / k - move cursor up / down"),
            Spans::from("  enter - select problem "),
            Spans::from(""),
            Spans::from(Self::bold("Solve Problem Menu".to_string())),
            Spans::from("  c - set compilation step"),
            Spans::from("  r - set run step"),
            Spans::from("  enter - run all examples / save compilation step / save run step"),
            Spans::from("  i - show detailed information about last run"),
            Spans::from("  f - run last failed example"),
        ];
        Paragraph::new(spans).wrap(Wrap { trim: false })
    }

    fn calculate_curr_problem_range(curr_prob_id: usize, row_n: usize) -> (usize, usize) {
        let mut end = row_n; // Index of last row to be shown
        while curr_prob_id >= end {
            end += row_n;
        }

        (end - row_n, end)
    }

    pub fn get_problems_to_select<'a>(&self, available_rows: usize) -> Vec<Paragraph<'a>> {
        let current_prob_id = self.model.curr_prob_id.get();
        let range = Self::calculate_curr_problem_range(current_prob_id, available_rows);

        let relevant_problems = self.problem_cacher.get_cached(range).unwrap_or({
            let problems = self.model.get_problems_in_range(range.0, range.1);
            self.problem_cacher.remember(range, &problems);
            problems
        });

        relevant_problems
            .into_iter()
            .enumerate()
            .map(|(relative_index, prob): (usize, Rc<Problem>)| {
                let true_index = relative_index + range.0;
                self.problem_as_row(prob.clone(), true_index, true_index == current_prob_id)
            })
            .collect()
    }

    pub fn problem_as_row<'a>(
        &self,
        prob: Rc<Problem>,
        id: usize,
        is_selected: bool,
    ) -> Paragraph<'a> {
        let prob = prob.clone();
        let spans = vec![
            Self::marker(is_selected, self.settings.pretty),
            Self::num(id),
            Self::bold(prob.problem_name.clone()),
            Self::difficulty(prob.difficulty.clone()),
        ];
        let par = Spans::from(spans);
        Paragraph::new(par).wrap(Wrap { trim: false })
    }

    fn num(id: usize) -> Span<'static> {
        Span::styled(
            format!("{id}. "),
            Style::default().fg(tui::style::Color::Cyan),
        )
    }

    fn difficulty<'a>(diff: Difficulty) -> Span<'a> {
        match diff {
            Difficulty::Easy => {
                Span::styled(" (Easy)  ", Style::default().fg(tui::style::Color::Green))
            }
            Difficulty::Medium => Span::styled(
                " (Medium) ",
                Style::default().fg(tui::style::Color::LightYellow),
            ),
            Difficulty::Hard => {
                Span::styled(" (Hard)  ", Style::default().fg(tui::style::Color::Red))
            }
        }
    }

    fn bold<'a>(text: String) -> Span<'a> {
        Span::styled(
            text,
            tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD),
        )
    }

    fn marker<'a>(is_selected: bool, is_pretty: bool) -> Span<'a> {
        if is_selected {
            let arrow = if is_pretty { " 🡆   " } else { " -->  " };
            return Span::styled(arrow, Style::default().fg(tui::style::Color::Cyan));
        }

        Span::from("")
    }

    pub fn curr_menu(&self) -> Menu {
        self.model.menu.get()
    }

    fn text<'a>(t: String) -> Span<'a> {
        Span::from(t)
    }

    pub fn detailed_problem<'a>(&self) -> (Paragraph, Paragraph, Paragraph) {
        let problem: &Problem = self.model.current_problem();
        let prob_name = Paragraph::new(Spans::from(Self::bold(problem.problem_name.clone())))
            .alignment(tui::layout::Alignment::Center);
        let prob_descr = Paragraph::new(Spans::from(Self::text(problem.problem_statement.clone())))
            .wrap(Wrap { trim: false });
        let prob_exmaple =
            Self::example(problem.examples.get(0).unwrap()).wrap(Wrap { trim: false });

        (prob_name, prob_descr, prob_exmaple)
    }

    fn example<'a>(exmp: &Example) -> Paragraph<'a> {
        Paragraph::new(vec![
            Spans::from(Self::bold("Example:".to_string())),
            Spans::from("input: \n\n".to_string() + &exmp.input),
            Spans::from("output: \n\n".to_string() + &exmp.output),
        ])
    }

    pub fn additional_data<'a>(&self) -> Paragraph<'a> {
        self.model.additional_data().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Menu {
    Select,
    Update,
    Solve,
    Help,
}

impl Default for Menu {
    fn default() -> Self { Self::Select }
}
// test tests::bench_pow ... bench:      13,345 ns/iter (+/- 1,405)
