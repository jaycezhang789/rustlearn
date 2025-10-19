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

pub fn pass_fail<'a>(
    records: &'a [(&'a str, u32)],
    pass_mark: f64,
) -> Result<HashMap<&'a str, bool>, &'static str> {
    let averages = student_average(records)?;
    let mut result = HashMap::new();
    for (name, average) in averages {
        result.insert(name, average >= pass_mark);
    }
    Ok(result)
}

pub fn class_median(records: &[(&str, u32)]) -> Result<f64, &'static str> {
    if records.is_empty() {
        return Err("没有提供分数记录");
    }
    let mut scores: Vec<u32> = records.iter().map(|(_, score)| *score).collect();
    scores.sort_unstable();
    let len = scores.len();
    if len % 2 == 1 {
        Ok(scores[len / 2] as f64)
    } else {
        let upper = scores[len / 2] as f64;
        let lower = scores[len / 2 - 1] as f64;
        Ok((lower + upper) / 2.0)
    }
}

pub fn grade_distribution(records: &[(&str, u32)]) -> HashMap<char, usize> {
    let mut distribution: HashMap<char, usize> =
        [('A', 0), ('B', 0), ('C', 0), ('D', 0), ('F', 0)]
            .into_iter()
            .collect();
    for &(_, score) in records {
        let grade = if score >= 90 {
            'A'
        } else if score >= 80 {
            'B'
        } else if score >= 70 {
            'C'
        } else if score >= 60 {
            'D'
        } else {
            'F'
        };
        *distribution.entry(grade).or_default() += 1;
    }
    distribution
}

pub fn normalize_scores<'a>(
    records: &'a [(&'a str, u32)],
) -> Result<HashMap<&'a str, f64>, &'static str> {
    if records.is_empty() {
        return Err("没有提供分数记录");
    }
    let total: u64 = records.iter().map(|(_, score)| *score as u64).sum();
    if total == 0 {
        return Err("总分为零，无法归一化");
    }
    let mut result = HashMap::new();
    for &(name, score) in records {
        result.insert(name, score as f64 / total as f64);
    }
    Ok(result)
}
