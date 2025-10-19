use std::cmp::Ordering;

pub fn welcome_message(name: &str) -> String {
    format!("欢迎你，{}！", name)
}

pub fn average_temperature(celsius: &[f64]) -> f64 {
    if celsius.is_empty() {
        return 0.0;
    }
    let sum: f64 = celsius.iter().copied().sum();
    (sum / celsius.len() as f64 * 100.0).round() / 100.0
}

pub fn merge_temperature_logs(mut primary: Vec<f64>, secondary: &[f64]) -> Vec<f64> {
    primary.extend_from_slice(secondary);
    primary.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    primary
}
