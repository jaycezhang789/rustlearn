use std::collections::HashMap;
use std::hash::Hash;

/// 难度 7：将 `(K, V)` 切片按键分组，返回 `HashMap<K, Vec<V>>`。
pub fn group_values<K, V>(pairs: &[(K, V)]) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    // 提示：使用 `entry` 与 `or_default`。
    todo!("把相同键的值收集到同一个向量")
}
