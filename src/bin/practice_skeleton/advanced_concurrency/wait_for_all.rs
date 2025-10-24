use std::thread;

/// 难度 7：等待一组线程句柄完成，返回成功完成的数量。
pub fn wait_for_all(handles: Vec<thread::JoinHandle<()>>) -> usize {
    // 提示：逐个 `join`，统计没有 panic 的线程数量。
    todo!("统计完成的线程数")
}
