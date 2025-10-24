//! 练习 4：枚举与模式匹配（层层深入）
//! 1. 枚举到字符串的格式化
//! 2. 字符串到枚举的解析
//! 3. 将命令序列应用到状态机
//! 4. 将命令序列序列化为文本
//! 5. 返回执行过程的历史记录
//! 6. 合并连续的同类命令进行优化
//! 7. 在执行过程中进行边界检查

pub mod command_type;
pub mod task01_describe_command;
pub mod task02_parse_command;
pub mod task03_run_program;
pub mod task04_serialize_program;
pub mod task05_execute_with_history;
pub mod task06_optimize_program;
pub mod task07_execute_with_limits;

pub use command_type::Command;
pub use task01_describe_command::describe_command;
pub use task02_parse_command::parse_command;
pub use task03_run_program::run_program;
pub use task04_serialize_program::serialize_program;
pub use task05_execute_with_history::execute_with_history;
pub use task06_optimize_program::optimize_program;
pub use task07_execute_with_limits::execute_with_limits;
