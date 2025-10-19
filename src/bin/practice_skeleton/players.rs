//! 练习 3：结构体与 `impl`（难度递增）
//! 1. 定义基础结构体并实现构造函数。
//! 2. 管理可变状态（积分增减）。
//! 3. 在切片上查找和返回结果。

#[allow(dead_code)]
pub struct Player {
    // TODO: 定义 `name: String` 与 `score: u32` 字段。
}

impl Player {
    /// 难度 1：创建一个初始积分为 0 的玩家。
    pub fn new(name: &str) -> Self {
        todo!("构造 Player 实例")
    }

    /// 难度 2：根据加分值更新积分并返回最新积分。
    pub fn add_score(&mut self, delta: u32) -> u32 {
        todo!("更新积分并返回新值")
    }

    /// 难度 3：根据扣分值减少积分并返回最新积分（不得为负）。
    pub fn apply_penalty(&mut self, penalty: u32) -> u32 {
        // 提示：可以使用 `saturating_sub` 避免下溢。
        todo!("扣除积分后返回当前积分")
    }
}

/// 额外挑战：在玩家列表中找到积分最高者，返回引用。
pub fn highest_scorer<'a>(players: &'a [Player]) -> Option<&'a Player> {
    // 提示：遍历切片并追踪当前最佳玩家。
    todo!("返回拥有最高积分的玩家引用")
}
