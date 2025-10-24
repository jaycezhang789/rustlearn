//! 练习 1：变量与所有权（由浅入深）
//! 1. 使用借用创建字符串
//! 2. 处理切片并返回浮点平均值
//! 3. 结合所有权与可变借用完成向量合并
//! 4. 在切片上查找最大值并返回 `Option`
//! 5. 综合生成统计报告
//! 6. 将温度数据标准化以便比较
//! 7. 计算相邻温度之间的变化趋势

pub mod task01_welcome_message;
pub mod task02_average_temperature;
pub mod task03_merge_temperature_logs;
pub mod task04_highest_temperature;
pub mod task05_summarize_temperatures;
pub mod task06_normalize_temperatures;
pub mod task07_temperature_trend;

pub use task01_welcome_message::welcome_message;
pub use task02_average_temperature::average_temperature;
pub use task03_merge_temperature_logs::merge_temperature_logs;
pub use task04_highest_temperature::highest_temperature;
pub use task05_summarize_temperatures::summarize_temperatures;
pub use task06_normalize_temperatures::normalize_temperatures;
pub use task07_temperature_trend::temperature_trend;
