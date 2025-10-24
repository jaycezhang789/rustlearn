//! 练习 2：可变变量与循环（逐步升级）
//! 1. 基本 while 循环
//! 2. 结合条件筛选的 for 循环
//! 3. 双层循环生成二维表
//! 4. 生成经典数列
//! 5. 构建帕斯卡三角形
//! 6. 计算阶乘
//! 7. 生成三角数列

pub mod countdown;
pub mod odd_squares;
pub mod multiplication_table;
pub mod fibonacci_sequence;
pub mod pascal_triangle;
pub mod factorial;
pub mod triangle_numbers;

pub use countdown::countdown;
pub use odd_squares::odd_squares;
pub use multiplication_table::multiplication_table;
pub use fibonacci_sequence::fibonacci_sequence;
pub use pascal_triangle::pascal_triangle;
pub use factorial::factorial;
pub use triangle_numbers::triangle_numbers;
