//! 练习 6：迭代器与闭包（难度递增）
//! 1. 简单过滤计数。
//! 2. 链式操作计算和。
//! 3. 去重、排序与收集。
//! 4. 使用 `scan` 生成前缀和。
//! 5. 分组拆分集合。
//! 6. 计算笛卡尔积。
//! 7. 检测第一个重复出现的元素。

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

/// 难度 4：返回输入序列的前缀和向量。
pub fn running_total(nums: &[i32]) -> Vec<i32> {
    // 提示：尝试使用 `iter().scan`.
    todo!("计算前缀和")
}

/// 难度 5：将数字按正数/非正数分组返回两个向量。
pub fn partition_by_sign(nums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    // 提示：使用 `iter().partition`.
    todo!("按符号分组")
}

/// 难度 6：生成两个数组的笛卡尔积，返回所有 `(a, b)` 配对。
pub fn cartesian_pairs(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    // 提示：可以使用双层迭代或者迭代器嵌套。
    todo!("返回所有配对")
}

/// 难度 7：找到序列中第一个重复出现的元素（再次出现时返回），没有则返回 `None`。
pub fn first_duplicate(nums: &[i32]) -> Option<i32> {
    // 提示：结合 `HashSet` 跟踪已出现的数字。
    todo!("检测第一个重复值")
}
