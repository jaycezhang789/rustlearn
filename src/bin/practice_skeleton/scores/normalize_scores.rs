use std::collections::HashMap;

/// 难度 7：返回每位学生在总分中的占比（总分为全部学生的分数之和）。
pub fn normalize_scores<'a>(
    records: &'a [(&'a str, u32)],
) -> Result<HashMap<&'a str, f64>, &'static str> {
    // 提示：先计算总分，再逐个除以总分。
    todo!("返回占比哈希表")
}
