#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

//! Rust 入门练习题（请在 TODO 标记处补全代码）
//!
//! 使用方法：
//! 1. 运行 `cargo run --bin practice_skeleton` 来编译检查。
//! 2. 逐个删除 `todo!()` 并补全逻辑。
//! 3. 修改后多次运行编译器，利用错误信息指引你熟悉语法。

#[path = "practice_skeleton/basics.rs"]
mod basics;
#[path = "practice_skeleton/commands.rs"]
mod commands;
#[path = "practice_skeleton/iterators.rs"]
mod iterators;
#[path = "practice_skeleton/loops.rs"]
mod loops;
#[path = "practice_skeleton/players.rs"]
mod players;
#[path = "practice_skeleton/scores.rs"]
mod scores;

pub use basics::{average_temperature, merge_temperature_logs, welcome_message};
pub use commands::{describe_command, parse_command, run_program, Command};
pub use iterators::{count_positive, even_square_sum, unique_sorted_evens};
pub use loops::{countdown, multiplication_table, odd_squares};
pub use players::{highest_scorer, Player};
pub use scores::{student_average, top_student, total_scores};

fn main() {
    println!("Rust 练习题骨架：请在对应模块的 TODO 标记处补全代码后运行测试。");
    // 你可以在这里添加临时调试代码来调用上述模块中的函数。
    // 完成后移除或注释掉调试代码即可。
}
