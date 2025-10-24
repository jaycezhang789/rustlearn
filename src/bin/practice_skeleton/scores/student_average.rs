use std::collections::HashMap;

/// 难度 2：给定 `(学生, 分数)` 键值对数组，统计每个学生的平均分。
/// - 如果分数列表为空，返回 `Err(&'static str)`。
pub fn student_average<'a>(
    records: &'a [(&'a str, u32)],
) -> Result<HashMap<&'a str, f64>, &'static str> {
    todo!("计算平均成绩并返回 HashMap")
}
