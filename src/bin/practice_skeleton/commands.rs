//! 练习 4：枚举与模式匹配（层层深入）
//! 1. 枚举到字符串的格式化。
//! 2. 字符串到枚举的解析。
//! 3. 将命令序列应用到状态机。
//! 4. 将命令序列序列化为文本。
//! 5. 返回执行过程的历史记录。
//! 6. 合并连续的同类命令进行优化。
//! 7. 在执行过程中进行边界检查。

#[derive(Debug, PartialEq)]
pub enum Command {
    Increment(i32),
    Decrement(i32),
    Reset,
}

/// 难度 1：把命令格式化为可读字符串。
pub fn describe_command(cmd: &Command) -> String {
    // 提示：使用 `match` 匹配不同枚举分支。
    todo!("返回描述命令的字符串")
}

/// 难度 2：解析文本命令。
pub fn parse_command(input: &str) -> Option<Command> {
    todo!("解析命令字符串")
}

/// 难度 3：从初始值开始依次执行命令，返回最终结果。
pub fn run_program(start: i32, commands: &[Command]) -> i32 {
    // 提示：遍历命令数组，根据分支调整累积值。
    todo!("执行所有命令并返回最终值")
}

/// 难度 4：把命令序列序列化为多行字符串。
pub fn serialize_program(commands: &[Command]) -> String {
    // 提示：为 `Vec<String>` 收集后用 `join("\n")` 拼接。
    todo!("返回文本表示")
}

/// 难度 5：执行命令并记录每一步值，返回历史列表（包含初始值）。
pub fn execute_with_history(start: i32, commands: &[Command]) -> Vec<i32> {
    // 提示：可以复用 `run_program` 的逻辑，但存储中间状态。
    todo!("返回执行过程的所有状态")
}

/// 难度 6：合并相邻的增减命令（同类型相加），忽略加/减 0 的命令。
pub fn optimize_program(commands: &[Command]) -> Vec<Command> {
    // 提示：遍历命令，累积当前增减量，当遇到不同类型或 reset 时输出。
    todo!("返回优化后的命令序列")
}

/// 难度 7：执行命令时确保数值始终位于 `[min, max]` 区间，超出范围则提前停止并返回当前值。
pub fn execute_with_limits(start: i32, commands: &[Command], min: i32, max: i32) -> i32 {
    // 提示：每次执行后检查是否越界，若越界立即返回。
    todo!("带边界检查的命令执行")
}
