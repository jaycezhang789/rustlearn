//! 高级练习：泛型与 Trait 约束
//! 1. 通过泛型函数返回克隆。
//! 2. 结合 `Default` 与 `Clone` 构造值。
//! 3. 使用泛型与哈希集合合并数据。
//! 4. 借助 `ToString` 实现类型到字符串的映射。
//! 5. 在泛型上下文中对键排序。
//! 6. 根据提取的键值返回最大元素。
//! 7. 将键值对分组收集。

use std::collections::HashMap;
use std::hash::Hash;

/// 难度 1：返回切片中的第一个元素克隆。
pub fn first_clone<T: Clone>(items: &[T]) -> Option<T> {
    // 提示：可以调用 `slice::first` 并配合 `cloned`。
    todo!("克隆第一个元素")
}

/// 难度 2：返回一个包含传入值与默认值的元组。
pub fn pair_with_default<T: Clone + Default>(value: T) -> (T, T) {
    // 提示：`T::default()` 可构造默认值。
    todo!("将 value 与默认值组成元组")
}

/// 难度 3：合并两个哈希表，优先使用 `right` 中的值覆盖 `left`。
pub fn merge_maps<K, V>(left: &HashMap<K, V>, right: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    // 提示：从 `left.clone()` 开始，再迭代 `right` 更新。
    todo!("返回合并后的 HashMap")
}

/// 难度 4：把任意实现 `ToString` 的集合映射为字符串列表。
pub fn to_string_vec<T: ToString>(items: &[T]) -> Vec<String> {
    // 提示：`iter().map(|item| item.to_string())`。
    todo!("返回字符串向量")
}

/// 难度 5：返回哈希表的有序键列表。
pub fn sorted_keys<K, V>(map: &HashMap<K, V>) -> Vec<K>
where
    K: Eq + Hash + Clone + Ord,
{
    // 提示：收集键后排序。
    todo!("返回排序后的键")
}

/// 难度 6：根据 `key_fn` 返回的键值选出最大元素。
pub fn max_by_key<T, K, F>(items: &[T], mut key_fn: F) -> Option<&T>
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    // 提示：遍历数据，跟踪当前最大键与元素引用。
    todo!("返回最大元素引用")
}

/// 难度 7：将 `(K, V)` 切片按键分组，返回 `HashMap<K, Vec<V>>`。
pub fn group_values<K, V>(pairs: &[(K, V)]) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    // 提示：使用 `entry` 和 `or_default`。
    todo!("把相同键的值收集到同一个向量")
}
