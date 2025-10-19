// Rust 入门练习题参考答案
//
// 可以运行 `cargo run --bin practice_solution` 验证实现。

pub mod advanced_generics;
pub mod advanced_lifetimes;
pub mod advanced_concurrency;
pub mod basics;
pub mod loops;
pub mod players;
pub mod commands;
pub mod scores;
pub mod iterators;

pub use basics::{
    average_temperature, highest_temperature, merge_temperature_logs, normalize_temperatures,
    summarize_temperatures, temperature_trend, welcome_message,
};
pub use loops::{
    countdown, factorial, fibonacci_sequence, multiplication_table, odd_squares, pascal_triangle,
    triangle_numbers,
};
pub use players::{
    award_bonus, average_score, find_player, highest_scorer, top_n_players, Player,
};
pub use commands::{
    describe_command, execute_with_history, execute_with_limits, optimize_program, parse_command,
    run_program, serialize_program, Command,
};
pub use scores::{
    class_median, grade_distribution, normalize_scores, pass_fail, student_average, total_scores,
    top_student,
};
pub use iterators::{
    cartesian_pairs, count_positive, even_square_sum, first_duplicate, partition_by_sign,
    running_total, unique_sorted_evens,
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
