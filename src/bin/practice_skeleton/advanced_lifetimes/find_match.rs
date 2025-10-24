/// 难度 5：在字符串集合中查找第一个满足条件的引用。
pub fn find_match<'a, F>(items: &'a [&'a str], predicate: F) -> Option<&'a str>
where
    F: Fn(&str) -> bool,
{
    // 提示：遍历并应用闭包；注意返回引用需满足生命周期。
    todo!("返回第一个匹配的字符串切片")
}
