/// 带生命周期的结构体，保存标题与正文引用。
pub struct Article<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
