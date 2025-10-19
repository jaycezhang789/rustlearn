//! 运行 `cargo run --bin practice_solution` 查看参考答案的输出。

#[path = "../../answers/mod.rs"]
mod answers;

use answers::{
    average_temperature, countdown, count_positive, describe_command, even_square_sum,
    highest_scorer, merge_temperature_logs, multiplication_table, odd_squares, parse_command,
    run_program, student_average, total_scores, top_student, unique_sorted_evens, welcome_message,
    Command, Player,
};

fn main() {
    println!("== 基础语法 ==");
    println!("{}", welcome_message("Ferris"));
    println!("平均温度: {}", average_temperature(&[18.2, 20.5, 21.0]));
    println!(
        "合并温度记录: {:?}",
        merge_temperature_logs(vec![19.0, 22.1], &[18.5, 25.0])
    );

    println!("\n== 循环练习 ==");
    println!("倒计时: {:?}", countdown(5));
    println!("奇数平方: {:?}", odd_squares(1, 10));
    println!("乘法表 1..=3: {:?}", multiplication_table(3));

    println!("\n== 结构体与方法 ==");
    let mut player = Player::new("Ferris");
    player.add_score(15);
    player.apply_penalty(3);
    println!("玩家: {} 积分 {}", player.name, player.score);
    let other = Player {
        name: "Rustacean".to_string(),
        score: 20,
    };
    let roster = vec![player, other];
    if let Some(best) = highest_scorer(&roster) {
        println!("最高分玩家: {} ({})", best.name, best.score);
    }

    println!("\n== 枚举与命令解析 ==");
    let cmd = parse_command("inc 42").expect("合法命令");
    println!("命令解析: {:?}, 描述: {}", cmd, describe_command(&cmd));
    let program = vec![cmd, Command::Decrement(10), Command::Reset, Command::Increment(5)];
    println!("执行命令后的结果: {}", run_program(0, &program));

    println!("\n== 集合与错误处理 ==");
    let records = [("Alice", 90), ("Bob", 75), ("Alice", 95)];
    println!("总分: {:?}", total_scores(&records));
    println!("平均分: {:?}", student_average(&records));
    println!("最佳学生: {:?}", top_student(&records));

    println!("\n== 迭代器与闭包 ==");
    let nums = [1, -3, 2, 4, 2, 6, 6];
    println!("正数个数: {}", count_positive(&nums));
    println!("偶数平方和: {}", even_square_sum(&[1, 2, 3, 4, 5, 6]));
    println!("唯一偶数: {:?}", unique_sorted_evens(&nums));
}
