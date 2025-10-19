//! 练习 6：迭代器与闭包（难度递增）
//! 1. 简单过滤计数。
//! 2. 链式操作计算和。
//! 3. 去重、排序与收集。

/// 难度 1：计算正整数的个数。
pub fn count_positive(nums: &[i32]) -> usize {
    // 提示：结合 `iter`、`filter` 与 `count` 或 `filter` + `count()`
    todo!("统计大于 0 的元素数量")
}

/// 难度 2：使用闭包过滤输入向量中的偶数，并返回这些偶数的平方和。
/// - 要求使用迭代器组合（`iter`、`filter`、`map`、`sum`）。
pub fn even_square_sum(nums: &[i32]) -> i32 {
    todo!("利用迭代器链实现")
}

/// 难度 3：收集所有偶数、去重并升序排序后返回新向量。
pub fn unique_sorted_evens(nums: &[i32]) -> Vec<i32> {
    // 提示：可以使用 `collect::<Vec<_>>()` 后配合 `sort` 与 `dedup`。
    todo!("返回去重排序后的偶数列表")
}
