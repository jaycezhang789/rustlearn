//! 练习 6：迭代器与闭包（难度递增）
//! 1. 简单过滤计数
//! 2. 链式操作计算和
//! 3. 去重、排序与收集
//! 4. 使用 `scan` 生成前缀和
//! 5. 分组拆分集合
//! 6. 计算笛卡尔积
//! 7. 检测第一个重复出现的元素

pub mod task01_count_positive;
pub mod task02_even_square_sum;
pub mod task03_unique_sorted_evens;
pub mod task04_running_total;
pub mod task05_partition_by_sign;
pub mod task06_cartesian_pairs;
pub mod task07_first_duplicate;

pub use task01_count_positive::count_positive;
pub use task02_even_square_sum::even_square_sum;
pub use task03_unique_sorted_evens::unique_sorted_evens;
pub use task04_running_total::running_total;
pub use task05_partition_by_sign::partition_by_sign;
pub use task06_cartesian_pairs::cartesian_pairs;
pub use task07_first_duplicate::first_duplicate;
