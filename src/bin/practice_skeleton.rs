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
#[path = "practice_skeleton/advanced_generics.rs"]
mod advanced_generics;
#[path = "practice_skeleton/advanced_lifetimes.rs"]
mod advanced_lifetimes;
#[path = "practice_skeleton/advanced_concurrency.rs"]
mod advanced_concurrency;

pub use basics::{
    average_temperature, highest_temperature, merge_temperature_logs, normalize_temperatures,
    summarize_temperatures, temperature_trend, welcome_message,
};
pub use commands::{
    describe_command, execute_with_history, execute_with_limits, optimize_program, parse_command,
    run_program, serialize_program, Command,
};
pub use iterators::{
    cartesian_pairs, count_positive, even_square_sum, first_duplicate, partition_by_sign,
    running_total, unique_sorted_evens,
};
pub use loops::{
    countdown, factorial, fibonacci_sequence, multiplication_table, odd_squares, pascal_triangle,
    triangle_numbers,
};
pub use players::{
    award_bonus, average_score, find_player, highest_scorer, top_n_players, Player,
};
pub use scores::{
    class_median, grade_distribution, normalize_scores, pass_fail, student_average, top_student,
    total_scores,
};
pub use advanced_generics::{
    first_clone, group_values, max_by_key, merge_maps, pair_with_default, sorted_keys,
    to_string_vec,
};
pub use advanced_lifetimes::{
    find_match, longest_with_note, longest_word, pick_longest, shortest_word, Article,
};
pub use advanced_concurrency::{
    chunked_sum, collect_messages, concurrent_counter, parallel_sum, run_tasks, spawn_sum,
    wait_for_all,
};

fn main() {
    println!("Rust 练习题骨架：请在对应模块的 TODO 标记处补全代码后运行测试。");
    // 你可以在这里添加临时调试代码来调用上述模块中的函数。
    // 完成后移除或注释掉调试代码即可。
}
