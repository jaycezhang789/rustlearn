//! 高级练习：借用与生命周期
//! 1. 返回更长的字符串引用
//! 2. 从句子中提取最长单词的切片
//! 3. 在结构体中保存带生命周期的引用
//! 4. 为结构体实现方法并返回带生命周期的值
//! 5. 结合闭包和生命周期在列表中查找匹配项
//! 6. 找出最短单词
//! 7. 带提示输出较长字符串
//! 8. 在结构体上实现更多引用方法

pub mod task01_pick_longest;
pub mod task02_longest_word;
pub mod task03_article;
pub mod task05_find_match;
pub mod task06_shortest_word;
pub mod task07_longest_with_note;

pub use task01_pick_longest::pick_longest;
pub use task02_longest_word::longest_word;
pub use task03_article::Article;
pub use task05_find_match::find_match;
pub use task06_shortest_word::shortest_word;
pub use task07_longest_with_note::longest_with_note;
