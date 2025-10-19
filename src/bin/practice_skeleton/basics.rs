//! 练习 1：变量与所有权（由浅入深）
//! 1. 使用借用创建字符串。
//! 2. 处理切片并返回浮点平均值。
//! 3. 结合所有权与可变借用完成向量合并。
//! 4. 在切片上查找最大值并返回 `Option`。
//! 5. 综合生成统计报告。
//! 6. 将温度数据标准化以便比较。
//! 7. 计算相邻温度之间的变化趋势。

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

/// 难度 4：返回最高温度，如果列表为空则返回 `None`。
pub fn highest_temperature(celsius: &[f64]) -> Option<f64> {
    // 提示：可以使用 `iter().copied().max_by`.
    todo!("查找最大温度")
}

/// 难度 5：生成城市温度报告，格式如 `"城市: 平均 xx.xx°C, 最高 yy.yy°C"`。
/// - 空列表时返回 `"城市: 暂无数据"`。
pub fn summarize_temperatures(city: &str, temps: &[f64]) -> String {
    // 提示：复用前面实现的平均值与最高值函数。
    todo!("返回格式化后的汇总字符串")
}

/// 难度 6：将所有温度减去基准值，返回标准化后的向量。
pub fn normalize_temperatures(temps: &[f64], baseline: f64) -> Vec<f64> {
    // 提示：遍历切片并对每个元素减去 baseline。
    todo!("返回标准化后的温度序列")
}

/// 难度 7：返回连续两个温度之间的变化量，长度应比输入少 1。
pub fn temperature_trend(temps: &[f64]) -> Vec<f64> {
    // 提示：使用窗口迭代或索引访问相邻元素。
    todo!("返回温度变化趋势")
}
