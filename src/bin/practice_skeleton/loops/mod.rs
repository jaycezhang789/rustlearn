//! 练习 2：可变变量与循环（逐步升级）
//! 1. 基本 while 循环
//! 2. 结合条件筛选的 for 循环
//! 3. 双层循环生成二维表
//! 4. 生成经典数列
//! 5. 构建帕斯卡三角形
//! 6. 计算阶乘
//! 7. 生成三角数列

pub mod task01_countdown;
pub mod task02_odd_squares;
pub mod task03_multiplication_table;
pub mod task04_fibonacci_sequence;
pub mod task05_pascal_triangle;
pub mod task06_factorial;
pub mod task07_triangle_numbers;

pub use task01_countdown::countdown;
pub use task02_odd_squares::odd_squares;
pub use task03_multiplication_table::multiplication_table;
pub use task04_fibonacci_sequence::fibonacci_sequence;
pub use task05_pascal_triangle::pascal_triangle;
pub use task06_factorial::factorial;
pub use task07_triangle_numbers::triangle_numbers;
