//! 练习 5：集合与错误处理（逐渐复杂）
//! 1. 累加分数
//! 2. 计算平均值
//! 3. 处理错误并挑选最佳学生
//! 4. 根据平均分判断是否及格
//! 5. 计算班级成绩中位数
//! 6. 统计成绩分布
//! 7. 将成绩按总分归一化

pub mod task01_total_scores;
pub mod task02_student_average;
pub mod task03_top_student;
pub mod task04_pass_fail;
pub mod task05_class_median;
pub mod task06_grade_distribution;
pub mod task07_normalize_scores;

pub use task01_total_scores::total_scores;
pub use task02_student_average::student_average;
pub use task03_top_student::top_student;
pub use task04_pass_fail::pass_fail;
pub use task05_class_median::class_median;
pub use task06_grade_distribution::grade_distribution;
pub use task07_normalize_scores::normalize_scores;
