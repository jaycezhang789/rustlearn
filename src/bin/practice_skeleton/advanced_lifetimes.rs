//! 高级练习：借用与生命周期
//! 1. 返回更长的字符串引用。
//! 2. 从句子中提取最长单词的切片。
//! 3. 在结构体中保存带生命周期的引用。
//! 4. 为结构体实现方法并返回带生命周期的值。
//! 5. 结合闭包和生命周期在列表中查找匹配项。
//! 6. 找出最短单词。
//! 7. 带提示输出较长字符串。
//! 8. 在结构体上实现更多引用方法。

/// 难度 1：返回两个字符串切片中较长的引用。
pub fn pick_longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    // 提示：比较长度后返回对应引用。
    todo!("返回较长的字符串切片")
}

/// 难度 2：返回句子中最长单词的切片。
pub fn longest_word<'a>(sentence: &'a str) -> Option<&'a str> {
    // 提示：使用 `split_whitespace` 并跟踪最长单词。
    todo!("找到最长单词")
}

/// 带生命周期的结构体，保存标题与正文引用。
pub struct Article<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> Article<'a> {
    /// 难度 3：构造一个 `Article`。
    pub fn new(title: &'a str, body: &'a str) -> Self {
        todo!("创建 Article 实例")
    }

    /// 难度 4：返回标题的前 30 个字符（或全部）。
    pub fn teaser(&self) -> &'a str {
        // 提示：可以用 `split_at` 或 `get(..limit)`。
        todo!("返回标题开头部分")
    }
}

/// 难度 5：在字符串集合中查找第一个满足条件的引用。
pub fn find_match<'a, F>(items: &'a [&'a str], predicate: F) -> Option<&'a str>
where
    F: Fn(&str) -> bool,
{
    // 提示：遍历并应用闭包；注意返回引用需满足生命周期。
    todo!("返回第一个匹配的字符串切片")
}

/// 难度 6：返回句子中最短的单词。
pub fn shortest_word<'a>(sentence: &'a str) -> Option<&'a str> {
    // 提示：与寻找最长类似，只是比较条件相反。
    todo!("找到最短单词")
}

/// 难度 7：打印提示信息后返回较长的字符串切片。
pub fn longest_with_note<'a>(left: &'a str, right: &'a str, note: &str) -> &'a str {
    // 提示：你可以先输出 note，再调用已有的逻辑。
    todo!("带提示返回较长字符串")
}

impl<'a> Article<'a> {
    /// 难度 8：返回正文的长度。
    pub fn body_length(&self) -> usize {
        todo!("返回正文长度")
    }
}
