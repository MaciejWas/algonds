use tui::widgets::Axis;
use tui::widgets::Chart;
use crate::application::ui::ProblemMenuLayout;
use crate::application::ui::UIElement;
use crate::application::View;
use tui::{
    backend::Backend,
    widgets::{Dataset, GraphType},
    style::{Style, Color},
    text::{Span},
    Frame,
    symbols,
};

pub struct PerformanceChart {
    data: Vec<(f64, f64)>,
}

impl UIElement for PerformanceChart {
    type ExpectedLayout = ProblemMenuLayout;

    fn setup(view: &View) -> Self {
        let data = view.performance();
        Self {
            data
        }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &ProblemMenuLayout) {
        let points = self.data;

        if points.is_empty() {
            return;
        }

        let dataset = Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .graph_type(GraphType::Line)
            .data(&points);

        let min_complexity = points[0].0;
        let max_complexity = points[points.len() - 1].0;

        let max_time: f64 = points.iter()
            .map(|(_, y)| y)
            .max_by(|a, b| a.total_cmp(b))
            .cloned()
            .unwrap_or(1.0) * 1.1;

        let chart = Chart::new(vec![dataset])
            .x_axis(Axis::default().title("log_2(Complexity)").bounds([min_complexity, max_complexity]).labels(vec![
                Span::from(format!("{:.2}", min_complexity)),
                Span::from(format!("{:.2}", ((min_complexity + max_complexity) / 2.0))),
                Span::from(format!("{:.2}", max_complexity)),
            ]))
            .y_axis(Axis::default().title("Time[s]").bounds([0.0, max_time]).labels(vec![
                Span::from("0.00"),
                Span::from(format!("{:.2}", max_time))
            ]));
        frame.render_widget(chart, layout.problem_tabs);
    }
}
