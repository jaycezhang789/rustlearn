//! 练习 6：迭代器与闭包（难度递增）
//! 1. 简单过滤计数
//! 2. 链式操作计算和
//! 3. 去重、排序与收集
//! 4. 使用 `scan` 生成前缀和
//! 5. 分组拆分集合
//! 6. 计算笛卡尔积
//! 7. 检测第一个重复出现的元素

pub mod count_positive;
pub mod even_square_sum;
pub mod unique_sorted_evens;
pub mod running_total;
pub mod partition_by_sign;
pub mod cartesian_pairs;
pub mod first_duplicate;

pub use count_positive::count_positive;
pub use even_square_sum::even_square_sum;
pub use unique_sorted_evens::unique_sorted_evens;
pub use running_total::running_total;
pub use partition_by_sign::partition_by_sign;
pub use cartesian_pairs::cartesian_pairs;
pub use first_duplicate::first_duplicate;
