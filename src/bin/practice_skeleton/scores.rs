//! 练习 5：集合与错误处理（逐渐复杂）
//! 1. 累加分数。
//! 2. 计算平均值。
//! 3. 处理错误并挑选最佳学生。

use std::collections::HashMap;

/// 难度 1：统计每位学生的总分。
pub fn total_scores<'a>(records: &'a [(&'a str, u32)]) -> HashMap<&'a str, u32> {
    // 提示：使用 `entry` API 叠加分数。
    todo!("返回每位学生的总分哈希表")
}

/// 难度 2：给定 `(学生, 分数)` 键值对数组，统计每个学生的平均分。
/// - 如果分数列表为空，返回 `Err(&'static str)`。
pub fn student_average<'a>(
    records: &'a [(&'a str, u32)],
) -> Result<HashMap<&'a str, f64>, &'static str> {
    todo!("计算平均成绩并返回 HashMap")
}

/// 难度 3：从平均分中找出成绩最高的学生。
/// - 如果原始记录为空，返回相同的错误。
pub fn top_student<'a>(
    records: &'a [(&'a str, u32)],
) -> Result<(&'a str, f64), &'static str> {
    // 提示：可以复用前面的 `student_average` 结果。
    todo!("返回最佳学生的名字与平均分")
}
