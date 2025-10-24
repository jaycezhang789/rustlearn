//! 练习 3：结构体与 `impl`（难度递增）
//! 1. 定义基础结构体并实现构造函数
//! 2. 管理可变状态（积分增减）
//! 3. 在切片上查找和返回结果
//! 4. 根据名字查找玩家
//! 5. 为达标玩家批量发放奖励
//! 6. 计算玩家平均积分
//! 7. 选出积分最高的前 N 名玩家

pub mod task01_player;
pub mod task03_highest_scorer;
pub mod task04_find_player;
pub mod task05_award_bonus;
pub mod task06_average_score;
pub mod task07_top_n_players;

pub use task01_player::Player;
pub use task03_highest_scorer::highest_scorer;
pub use task04_find_player::find_player;
pub use task05_award_bonus::award_bonus;
pub use task06_average_score::average_score;
pub use task07_top_n_players::top_n_players;
