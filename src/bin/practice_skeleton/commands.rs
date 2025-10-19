//! 练习 4：枚举与模式匹配（层层深入）
//! 1. 枚举到字符串的格式化。
//! 2. 字符串到枚举的解析。
//! 3. 将命令序列应用到状态机。

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
