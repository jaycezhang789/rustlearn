//! 高级练习：泛型与 Trait 约束
//! 1. 通过泛型函数返回克隆
//! 2. 结合 `Default` 与 `Clone` 构造值
//! 3. 使用泛型与哈希集合合并数据
//! 4. 借助 `ToString` 实现类型到字符串的映射
//! 5. 在泛型上下文中对键排序
//! 6. 根据提取的键值返回最大元素
//! 7. 将键值对分组收集

pub mod task01_first_clone;
pub mod task02_pair_with_default;
pub mod task03_merge_maps;
pub mod task04_to_string_vec;
pub mod task05_sorted_keys;
pub mod task06_max_by_key;
pub mod task07_group_values;

pub use task01_first_clone::first_clone;
pub use task02_pair_with_default::pair_with_default;
pub use task03_merge_maps::merge_maps;
pub use task04_to_string_vec::to_string_vec;
pub use task05_sorted_keys::sorted_keys;
pub use task06_max_by_key::max_by_key;
pub use task07_group_values::group_values;
