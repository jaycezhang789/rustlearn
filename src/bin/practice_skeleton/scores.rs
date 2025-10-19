//! 练习 5：集合与错误处理（逐渐复杂）
//! 1. 累加分数。
//! 2. 计算平均值。
//! 3. 处理错误并挑选最佳学生。
//! 4. 根据平均分判断是否及格。
//! 5. 计算班级成绩中位数。
//! 6. 统计成绩分布。
//! 7. 将成绩按总分归一化。

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

/// 难度 4：返回每位学生是否达到及格分数线（基于平均分计算）。
pub fn pass_fail<'a>(
    records: &'a [(&'a str, u32)],
    pass_mark: f64,
) -> Result<HashMap<&'a str, bool>, &'static str> {
    // 提示：先调用 `student_average` 再构造布尔结果。
    todo!("返回每位学生是否及格")
}

/// 难度 5：计算整体成绩的中位数（若记录为空则返回错误）。
pub fn class_median(records: &[(&str, u32)]) -> Result<f64, &'static str> {
    // 提示：复制到可变向量后排序，再计算中位数。
    todo!("返回班级成绩的中位数")
}

/// 难度 6：按照分段统计成绩人数（例如 A≥90，B≥80，C≥70，D≥60，F 其他）。
pub fn grade_distribution(records: &[(&str, u32)]) -> HashMap<char, usize> {
    // 提示：初始化计数并在遍历时累加。
    todo!("返回分布情况")
}

/// 难度 7：返回每位学生在总分中的占比（总分为全部学生的分数之和）。
pub fn normalize_scores<'a>(
    records: &'a [(&'a str, u32)],
) -> Result<HashMap<&'a str, f64>, &'static str> {
    // 提示：先计算总分，再逐个除以总分。
    todo!("返回占比哈希表")
}
