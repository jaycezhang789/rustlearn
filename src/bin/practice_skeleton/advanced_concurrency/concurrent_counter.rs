#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
use std::thread;

/// 难度 4：多个线程安全更新计数器。
pub fn concurrent_counter(tasks: usize, increments: usize) -> usize {
    // 提示：`Arc<Mutex<usize>>`，每个线程循环增加多次。
    todo!("返回最终计数")
}
