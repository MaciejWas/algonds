use crate::complexity::Function;
use std::iter::zip;

fn verify(
    expected_complexity: Box<dyn Function>,
    x: Vec<u32>,
    y: Vec<u32>,
) -> Result<f64, linreg::Error> {
    let codomain: Vec<u32> = y
        .iter()
        .map(|y| expected_complexity.closest_inverse(y))
        .collect();

    let fitting_x: Vec<u32> = x.iter().take(10).cloned().collect();
    let fitting_y: Vec<u32> = codomain.iter().take(10).cloned().collect();

    let test_x: Vec<u32> = x.iter().skip(10).cloned().collect();
    let test_y: Vec<u32> = codomain.iter().skip(10).cloned().collect();

    let (intercept, slope): (f64, f64) = linreg::linear_regression(&fitting_x, &fitting_y)?;

    Ok(zip(test_x, test_y)
        .into_iter()
        .map(|(x, y)| find_err(intercept + x as f64 * slope, y as f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()) // TODO: Remove unwrap
}

fn find_err(expected: f64, real: f64) -> f64 {
    f64::max(expected, real) / expected - 1.0
}
