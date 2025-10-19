use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn spawn_sum(nums: Vec<i64>) -> thread::JoinHandle<i64> {
    thread::spawn(move || nums.into_iter().sum())
}

pub fn parallel_sum(nums: &[i64]) -> i64 {
    if nums.is_empty() {
        return 0;
    }
    let mid = nums.len() / 2;
    let (left, right) = nums.split_at(mid);
    let left_vec = left.to_vec();
    let right_vec = right.to_vec();

    let left_handle = thread::spawn(move || left_vec.into_iter().sum::<i64>());
    let right_handle = thread::spawn(move || right_vec.into_iter().sum::<i64>());

    let left_sum = left_handle.join().unwrap();
    let right_sum = right_handle.join().unwrap();

    left_sum + right_sum
}

pub fn collect_messages(messages: &[String]) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    for msg in messages {
        let tx_clone = tx.clone();
        tx_clone.send(msg.clone()).unwrap();
    }
    drop(tx);
    rx.into_iter().collect()
}

pub fn concurrent_counter(tasks: usize, increments: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::with_capacity(tasks);
    for _ in 0..tasks {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments {
                let mut guard = counter_clone.lock().unwrap();
                *guard += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_count = *counter.lock().unwrap();
    final_count
}

pub fn run_tasks<T>(tasks: Vec<Box<dyn Fn() -> T + Send>>) -> Vec<T>
where
    T: Send + 'static,
{
    let mut handles = Vec::with_capacity(tasks.len());
    for task in tasks {
        handles.push(thread::spawn(move || task()));
    }
    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}

pub fn chunked_sum(nums: Vec<i64>, chunk_size: usize) -> i64 {
    if chunk_size == 0 || nums.is_empty() {
        return nums.into_iter().sum();
    }
    let mut handles = Vec::new();
    let mut start = 0;
    while start < nums.len() {
        let end = (start + chunk_size).min(nums.len());
        let chunk = nums[start..end].to_vec();
        handles.push(thread::spawn(move || chunk.into_iter().sum::<i64>()));
        start = end;
    }
    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}

pub fn wait_for_all(handles: Vec<thread::JoinHandle<()>>) -> usize {
    let mut completed = 0;
    for handle in handles {
        if handle.join().is_ok() {
            completed += 1;
        }
    }
    completed
}
