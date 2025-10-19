//! 练习 3：结构体与 `impl`（难度递增）
//! 1. 定义基础结构体并实现构造函数。
//! 2. 管理可变状态（积分增减）。
//! 3. 在切片上查找和返回结果。
//! 4. 根据名字查找玩家。
//! 5. 为达标玩家批量发放奖励。
//! 6. 计算玩家平均积分。
//! 7. 选出积分最高的前 N 名玩家。

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

/// 难度 4：查找名字匹配的玩家并返回引用。
pub fn find_player<'a>(players: &'a [Player], name: &str) -> Option<&'a Player> {
    // 提示：注意字符串比较需要借用。
    todo!("根据名字查找玩家")
}

/// 难度 5：为积分达到 `min_score` 的玩家增加奖励分，返回发放奖励的人数。
pub fn award_bonus(players: &mut [Player], min_score: u32, bonus: u32) -> usize {
    // 提示：遍历可变切片并计数。
    todo!("批量奖励玩家")
}

/// 难度 6：返回玩家平均积分，若列表为空返回 `None`。
pub fn average_score(players: &[Player]) -> Option<f64> {
    // 提示：遍历累加后除以人数。
    todo!("计算玩家平均积分")
}

/// 难度 7：返回积分最高的前 `n` 名玩家引用（若玩家不足则返回全部）。
pub fn top_n_players<'a>(players: &'a [Player], n: usize) -> Vec<&'a Player> {
    // 提示：可以复制索引排序，或使用迭代器和排序。
    todo!("筛选出前 N 名玩家")
}
