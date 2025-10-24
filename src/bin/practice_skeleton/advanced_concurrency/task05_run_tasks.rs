#[allow(unused_imports)]
use std::thread;

/// 难度 5：实现一个简单的任务执行器，顺序运行所有闭包并返回结果向量。
pub fn run_tasks<T>(tasks: Vec<Box<dyn Fn() -> T + Send>>) -> Vec<T>
where
    T: Send + 'static,
{
    // 提示：可以为每个任务 `spawn` 一个线程，再收集 `JoinHandle`。
    todo!("并发执行所有任务")
}
