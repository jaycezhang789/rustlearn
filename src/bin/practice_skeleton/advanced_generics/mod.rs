//! 高级练习：泛型与 Trait 约束
//! 1. 通过泛型函数返回克隆
//! 2. 结合 `Default` 与 `Clone` 构造值
//! 3. 使用泛型与哈希集合合并数据
//! 4. 借助 `ToString` 实现类型到字符串的映射
//! 5. 在泛型上下文中对键排序
//! 6. 根据提取的键值返回最大元素
//! 7. 将键值对分组收集

pub mod first_clone;
pub mod pair_with_default;
pub mod merge_maps;
pub mod to_string_vec;
pub mod sorted_keys;
pub mod max_by_key;
pub mod group_values;

pub use first_clone::first_clone;
pub use pair_with_default::pair_with_default;
pub use merge_maps::merge_maps;
pub use to_string_vec::to_string_vec;
pub use sorted_keys::sorted_keys;
pub use max_by_key::max_by_key;
pub use group_values::group_values;
