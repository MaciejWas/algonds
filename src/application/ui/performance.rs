use tui::widgets::Axis;
use tui::widgets::Chart;
use crate::application::common::*;
use crate::application::ui::ProblemScreenLayout;
use crate::application::ui::UIElement;
use crate::application::View;
use tui::widgets::Paragraph;
use tui::{
    backend::Backend,
    widgets::{Dataset, GraphType},
    style::{Modifier, Style, Color},
    text::{Span, Spans},
    Frame,
    symbols,
};

pub struct PerformanceChart {
    data: Vec<(f64, f64)>,
}

impl UIElement for PerformanceChart {
    type ExpectedLayout = ProblemScreenLayout;

    fn setup(view: &View) -> Self {
        let data = view.performance();
        Self {
            data
        }
    }

    fn render<B: Backend>(self, frame: &mut Frame<B>, layout: &ProblemScreenLayout) {
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
            .map(|(x, y)| y)
            .max_by(|a, b| a.total_cmp(b))
            .cloned()
            .unwrap_or(1.0) * 1.1;

        let chart = Chart::new(vec![dataset])
            .x_axis(Axis::default().title("Complexity").bounds([min_complexity, max_complexity]).labels(vec![
                Span::from(min_complexity.to_string()),
                Span::from(((min_complexity + max_complexity) / 2.0).to_string()),
                Span::from(max_complexity.to_string()),
            ]))
            .y_axis(Axis::default().title("Time").bounds([0.0, max_time]).labels(vec![
                Span::from("0.000s"),
                Span::from(format!("{:.3}", max_time))
            ]));
        frame.render_widget(chart, layout.data);
    }
}
