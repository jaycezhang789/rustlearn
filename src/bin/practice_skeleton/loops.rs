//! 练习 2：可变变量与循环（逐步升级）
//! 1. 基本 while 循环。
//! 2. 结合条件筛选的 for 循环。
//! 3. 双层循环生成二维表。
//! 4. 生成经典数列。
//! 5. 构建帕斯卡三角形。
//! 6. 计算阶乘。
//! 7. 生成三角数列。

/// 难度 1：从给定数字开始倒计时到 0（包含）。
pub fn countdown(start: u32) -> Vec<u32> {
    // 提示：可以使用 `while` 循环以及 `push`。
    todo!("返回从 start 到 0 的倒序列表")
}

/// 难度 2：生成 `[start, end)` 区间内所有奇数的平方，并以向量返回。
/// - 当 `start >= end` 时返回空向量。
pub fn odd_squares(start: i32, end: i32) -> Vec<i32> {
    // 提示：可以使用 `while` 循环或 `for` + 迭代器过滤。
    todo!("构造奇数平方列表")
}

/// 难度 3：生成 `size x size` 的乘法表。
/// - 第 i 行第 j 列的值为 `(i + 1) * (j + 1)`。
pub fn multiplication_table(size: u32) -> Vec<Vec<u32>> {
    // 提示：可以嵌套循环，也可结合迭代器。
    todo!("返回乘法表的二维向量表示")
}

/// 难度 4：返回长度为 `n` 的斐波那契数列（从 0,1 开始）。
pub fn fibonacci_sequence(n: u32) -> Vec<u64> {
    // 提示：跟踪前两个数，循环中推入新值。
    todo!("构造斐波那契序列")
}

/// 难度 5：生成 `rows` 行的帕斯卡三角形。
pub fn pascal_triangle(rows: usize) -> Vec<Vec<u64>> {
    // 提示：每一行可由上一行相邻元素求和得到。
    todo!("返回帕斯卡三角形表示")
}

/// 难度 6：计算 `n!`，使用 `u128` 存储结果。
pub fn factorial(n: u32) -> u128 {
    // 提示：从 1 开始累乘直到 n。
    todo!("计算阶乘")
}

/// 难度 7：生成前 `count` 个三角数（第 n 个为 `n * (n + 1) / 2`）。
pub fn triangle_numbers(count: usize) -> Vec<u64> {
    // 提示：使用递增累加器。
    todo!("返回三角数列")
}
