#[allow(dead_code)]
pub struct Player {
    // TODO: 定义 `name: String` 和 `score: u32` 字段。
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
