//! 练习 1：变量与所有权（由浅入深）
//! 1. 使用借用创建字符串
//! 2. 处理切片并返回浮点平均值
//! 3. 结合所有权与可变借用完成向量合并
//! 4. 在切片上查找最大值并返回 `Option`
//! 5. 综合生成统计报告
//! 6. 将温度数据标准化以便比较
//! 7. 计算相邻温度之间的变化趋势

pub mod welcome_message;
pub mod average_temperature;
pub mod merge_temperature_logs;
pub mod highest_temperature;
pub mod summarize_temperatures;
pub mod normalize_temperatures;
pub mod temperature_trend;

pub use welcome_message::welcome_message;
pub use average_temperature::average_temperature;
pub use merge_temperature_logs::merge_temperature_logs;
pub use highest_temperature::highest_temperature;
pub use summarize_temperatures::summarize_temperatures;
pub use normalize_temperatures::normalize_temperatures;
pub use temperature_trend::temperature_trend;
