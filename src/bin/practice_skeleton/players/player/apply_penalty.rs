use super::Player;

impl Player {
    /// 难度 3：根据扣分值减少积分并返回最新积分（不得为负）。
    pub fn apply_penalty(&mut self, penalty: u32) -> u32 {
        // 提示：可以使用 `saturating_sub` 避免下溢。
        todo!("扣除积分后返回当前积分")
    }
}
