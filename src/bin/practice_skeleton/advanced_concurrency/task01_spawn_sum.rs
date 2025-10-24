use std::thread;

/// 难度 1：启动一个线程计算向量元素之和，返回 `JoinHandle`。
pub fn spawn_sum(nums: Vec<i64>) -> thread::JoinHandle<i64> {
    // 提示：`thread::spawn` 接受闭包，注意移动 `nums`。
    todo!("在线程中求和")
}
