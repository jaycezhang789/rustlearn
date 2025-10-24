use std::collections::HashMap;
use std::hash::Hash;

/// 难度 5：返回哈希表的有序键列表。
pub fn sorted_keys<K, V>(map: &HashMap<K, V>) -> Vec<K>
where
    K: Eq + Hash + Clone + Ord,
{
    // 提示：收集键后排序。
    todo!("返回排序后的键")
}
