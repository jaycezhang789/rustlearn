//! 练习 1：变量与所有权（由浅入深）
//! 1. 使用借用创建字符串。
//! 2. 处理切片并返回浮点平均值。
//! 3. 结合所有权与可变借用完成向量合并。

/// 难度 1：根据传入的名字生成欢迎语。
/// - 输入是借用的 `&str`，返回拥有所有权的 `String`。
pub fn welcome_message(name: &str) -> String {
    // 提示：使用 `format!` 宏组合字符串。
    todo!("返回一条包含名字的欢迎语")
}

/// 难度 2：根据给定的温度列表，返回摄氏温度的平均值。
/// - 平均值需要保留两位小数。
pub fn average_temperature(celsius: &[f64]) -> f64 {
    // 提示：使用迭代器求和，注意浮点除法。
    todo!("返回平均温度，缺省值可用 0.0 占位")
}

/// 难度 3：合并主温度记录与备用记录。
/// - `primary` 拥有所有权，你可以直接修改它。
/// - `secondary` 以切片形式提供，需要按需借用或复制。
/// - 返回合并后的新向量，要求升序排列。
pub fn merge_temperature_logs(mut primary: Vec<f64>, secondary: &[f64]) -> Vec<f64> {
    // 提示：考虑使用 `extend_from_slice` 与 `sort_by`.
    todo!("合并两个数据源并返回排序后的向量")
}
