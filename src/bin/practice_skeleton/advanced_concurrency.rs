//! 高级练习：并发与多线程
//! 1. 在线程中执行计算并返回句柄。
//! 2. 拆分数据并行计算。
//! 3. 使用通道发送多条消息。
//! 4. 组合 `Mutex` 与多个线程安全更新共享状态。
//! 5. 实现一个简单的任务执行器。
//! 6. 按块划分任务并行求和。
//! 7. 等待所有线程完成并统计数量。

#[allow(unused_imports)]
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// 难度 1：启动一个线程计算向量元素之和，返回 `JoinHandle`。
pub fn spawn_sum(nums: Vec<i64>) -> thread::JoinHandle<i64> {
    // 提示：`thread::spawn` 接受闭包，注意移动 `nums`。
    todo!("在线程中求和")
}

/// 难度 2：并行拆分切片，使用两个线程计算左右区间之和并合并。
pub fn parallel_sum(nums: &[i64]) -> i64 {
    // 提示：使用 `split_at`，启动两个线程并 `join`。
    todo!("并行求和")
}

/// 难度 3：通过通道发送多条字符串消息，并收集到向量。
pub fn collect_messages(messages: &[String]) -> Vec<String> {
    // 提示：`mpsc::channel` + 克隆发送端。
    todo!("发送并接收所有消息")
}

/// 难度 4：多个线程安全更新计数器。
pub fn concurrent_counter(tasks: usize, increments: usize) -> usize {
    // 提示：`Arc<Mutex<usize>>`，每个线程循环增加多次。
    todo!("返回最终计数")
}

/// 难度 5：实现一个简单的任务执行器，顺序运行所有闭包并返回结果向量。
pub fn run_tasks<T>(tasks: Vec<Box<dyn Fn() -> T + Send>>) -> Vec<T>
where
    T: Send + 'static,
{
    // 提示：可以为每个任务 `spawn` 一个线程，再收集 `JoinHandle`。
    todo!("并发执行所有任务")
}

/// 难度 6：把数据按块拆分，每个块交给线程求和，最后汇总所有块的结果。
pub fn chunked_sum(nums: Vec<i64>, chunk_size: usize) -> i64 {
    // 提示：使用 `chunks` 迭代器，每个块 spawn 一个线程。
    todo!("按块并行求和")
}

/// 难度 7：等待一组线程句柄完成，返回成功完成的数量。
pub fn wait_for_all(handles: Vec<thread::JoinHandle<()>>) -> usize {
    // 提示：逐个 `join`，统计没有 panic 的线程数量。
    todo!("统计完成的线程数")
}
