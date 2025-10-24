//! 高级练习：并发与多线程
//! 1. 在线程中执行计算并返回句柄
//! 2. 拆分数据并行计算
//! 3. 使用通道发送多条消息
//! 4. 组合 `Mutex` 与多个线程安全更新共享状态
//! 5. 实现一个简单的任务执行器
//! 6. 按块划分任务并行求和
//! 7. 等待所有线程完成并统计数量

pub mod task01_spawn_sum;
pub mod task02_parallel_sum;
pub mod task03_collect_messages;
pub mod task04_concurrent_counter;
pub mod task05_run_tasks;
pub mod task06_chunked_sum;
pub mod task07_wait_for_all;

pub use task01_spawn_sum::spawn_sum;
pub use task02_parallel_sum::parallel_sum;
pub use task03_collect_messages::collect_messages;
pub use task04_concurrent_counter::concurrent_counter;
pub use task05_run_tasks::run_tasks;
pub use task06_chunked_sum::chunked_sum;
pub use task07_wait_for_all::wait_for_all;
