use std::collections::HashMap;
use std::hash::Hash;

/// 难度 3：合并两个哈希表，优先使用 `right` 中的值覆盖 `left`。
pub fn merge_maps<K, V>(left: &HashMap<K, V>, right: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    // 提示：从 `left.clone()` 开始，再迭代 `right` 更新。
    todo!("返回合并后的 HashMap")
}
