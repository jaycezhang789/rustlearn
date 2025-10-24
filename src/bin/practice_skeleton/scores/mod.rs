//! 练习 5：集合与错误处理（逐渐复杂）
//! 1. 累加分数
//! 2. 计算平均值
//! 3. 处理错误并挑选最佳学生
//! 4. 根据平均分判断是否及格
//! 5. 计算班级成绩中位数
//! 6. 统计成绩分布
//! 7. 将成绩按总分归一化

pub mod total_scores;
pub mod student_average;
pub mod top_student;
pub mod pass_fail;
pub mod class_median;
pub mod grade_distribution;
pub mod normalize_scores;

pub use total_scores::total_scores;
pub use student_average::student_average;
pub use top_student::top_student;
pub use pass_fail::pass_fail;
pub use class_median::class_median;
pub use grade_distribution::grade_distribution;
pub use normalize_scores::normalize_scores;
