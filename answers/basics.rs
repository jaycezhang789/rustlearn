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

pub fn highest_temperature(celsius: &[f64]) -> Option<f64> {
    celsius.iter().copied().fold(None, |max, value| match max {
        Some(current) => match value.partial_cmp(&current) {
            Some(Ordering::Greater) => Some(value),
            _ => Some(current),
        },
        None => Some(value),
    })
}

pub fn summarize_temperatures(city: &str, temps: &[f64]) -> String {
    if temps.is_empty() {
        return format!("{city}: 暂无数据");
    }
    let average = average_temperature(temps);
    let highest = highest_temperature(temps).unwrap_or(average);
    format!("{city}: 平均 {average:.2}°C, 最高 {highest:.2}°C")
}

pub fn normalize_temperatures(temps: &[f64], baseline: f64) -> Vec<f64> {
    temps.iter().map(|value| value - baseline).collect()
}

pub fn temperature_trend(temps: &[f64]) -> Vec<f64> {
    if temps.len() < 2 {
        return Vec::new();
    }
    temps
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}
