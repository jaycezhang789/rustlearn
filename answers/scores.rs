use std::cmp::Ordering;
use std::collections::HashMap;

pub fn total_scores<'a>(records: &'a [(&'a str, u32)]) -> HashMap<&'a str, u32> {
    let mut totals = HashMap::new();
    for &(name, score) in records {
        let entry = totals.entry(name).or_insert(0u32);
        *entry = (*entry).saturating_add(score);
    }
    totals
}

pub fn student_average<'a>(
    records: &'a [(&'a str, u32)],
) -> Result<HashMap<&'a str, f64>, &'static str> {
    if records.is_empty() {
        return Err("没有提供分数记录");
    }

    let mut totals: HashMap<&'a str, (u32, u32)> = HashMap::new();
    for &(name, score) in records {
        let entry = totals.entry(name).or_insert((0, 0));
        entry.0 = entry.0.saturating_add(score);
        entry.1 += 1;
    }

    let averages = totals
        .into_iter()
        .map(|(name, (total, count))| (name, total as f64 / count as f64))
        .collect();

    Ok(averages)
}

pub fn top_student<'a>(
    records: &'a [(&'a str, u32)],
) -> Result<(&'a str, f64), &'static str> {
    let averages = student_average(records)?;
    averages
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
        .ok_or("没有提供分数记录")
}
