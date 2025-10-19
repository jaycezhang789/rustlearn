//! 练习 2：可变变量与循环（逐步升级）
//! 1. 基本 while 循环。
//! 2. 结合条件筛选的 for 循环。
//! 3. 双层循环生成二维表。

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
