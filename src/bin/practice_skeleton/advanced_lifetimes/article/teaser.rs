use super::Article;

impl<'a> Article<'a> {
    /// 难度 4：返回标题的前 30 个字符（或全部）。
    pub fn teaser(&self) -> &'a str {
        // 提示：可以用 `split_at` 或 `get(..limit)`。
        todo!("返回标题开头部分")
    }
}
