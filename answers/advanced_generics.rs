use std::collections::HashMap;
use std::hash::Hash;

pub fn first_clone<T: Clone>(items: &[T]) -> Option<T> {
    items.first().cloned()
}

pub fn pair_with_default<T: Clone + Default>(value: T) -> (T, T) {
    (value.clone(), T::default())
}

pub fn merge_maps<K, V>(left: &HashMap<K, V>, right: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    let mut merged = left.clone();
    for (key, value) in right {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

pub fn to_string_vec<T: ToString>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

pub fn sorted_keys<K, V>(map: &HashMap<K, V>) -> Vec<K>
where
    K: Eq + Hash + Clone + Ord,
{
    let mut keys: Vec<K> = map.keys().cloned().collect();
    keys.sort();
    keys
}

pub fn max_by_key<T, K, F>(items: &[T], mut key_fn: F) -> Option<&T>
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    let mut iter = items.iter();
    let first = iter.next()?;
    let mut best_item = first;
    let mut best_key = key_fn(best_item);
    for item in iter {
        let key = key_fn(item);
        if key > best_key {
            best_key = key;
            best_item = item;
        }
    }
    Some(best_item)
}

pub fn group_values<K, V>(pairs: &[(K, V)]) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    let mut grouped: HashMap<K, Vec<V>> = HashMap::new();
    for (key, value) in pairs {
        grouped.entry(key.clone()).or_default().push(value.clone());
    }
    grouped
}
