// Rust 入门练习题参考答案
//
// 可以运行 `cargo run --bin practice_solution` 验证实现。

pub mod basics;
pub mod loops;
pub mod players;
pub mod commands;
pub mod scores;
pub mod iterators;

pub use basics::{average_temperature, merge_temperature_logs, welcome_message};
pub use loops::{countdown, multiplication_table, odd_squares};
pub use players::{highest_scorer, Player};
pub use commands::{describe_command, parse_command, run_program, Command};
pub use scores::{student_average, total_scores, top_student};
pub use iterators::{count_positive, even_square_sum, unique_sorted_evens};
