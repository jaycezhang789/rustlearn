/// 难度 6：根据 `key_fn` 返回的键值选出最大元素。
pub fn max_by_key<T, K, F>(items: &[T], mut key_fn: F) -> Option<&T>
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    // 提示：遍历数据，跟踪当前最大键与元素引用。
    todo!("返回最大元素引用")
}
