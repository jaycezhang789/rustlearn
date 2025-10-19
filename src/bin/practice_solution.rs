//! 运行 `cargo run --bin practice_solution` 查看参考答案的输出。

#[path = "../../answers/mod.rs"]
mod answers;

use std::{collections::HashMap, thread};

use answers::{
    Article, award_bonus, average_score, average_temperature, class_median, collect_messages,
    concurrent_counter, countdown, count_positive, describe_command, even_square_sum,
    execute_with_history, execute_with_limits, factorial, fibonacci_sequence, find_match,
    find_player, first_clone, first_duplicate, grade_distribution, group_values, highest_scorer,
    highest_temperature, longest_with_note, longest_word, max_by_key, merge_maps,
    merge_temperature_logs, multiplication_table, normalize_scores, normalize_temperatures,
    odd_squares, optimize_program, pair_with_default, parallel_sum, partition_by_sign, pass_fail,
    pascal_triangle, parse_command, pick_longest, run_program, run_tasks, running_total,
    serialize_program, shortest_word, sorted_keys, spawn_sum, student_average, summarize_temperatures,
    temperature_trend, to_string_vec, top_n_players, top_student, total_scores, triangle_numbers,
    unique_sorted_evens, wait_for_all, welcome_message, cartesian_pairs, chunked_sum, Command, Player,
};

fn main() {
    println!("== 基础语法 ==");
    let temps = [18.2, 20.5, 21.0, 25.3];
    println!("{}", welcome_message("Ferris"));
    println!("平均温度: {}", average_temperature(&temps));
    println!("最高温度: {:?}", highest_temperature(&temps));
    println!(
        "合并温度记录: {:?}",
        merge_temperature_logs(vec![19.0, 22.1], &[18.5, 25.0])
    );
    println!(
        "温度总结: {}",
        summarize_temperatures("上海", &temps)
    );
    println!("标准化温度: {:?}", normalize_temperatures(&temps, 20.0));
    println!("温度变化: {:?}", temperature_trend(&temps));

    println!("\n== 泛型与 Trait ==");
    let numbers = vec![1, 2, 3, 4];
    println!("第一个元素克隆: {:?}", first_clone(&numbers));
    println!("与默认值配对: {:?}", pair_with_default(42));
    let mut left_map: HashMap<&str, i32> = HashMap::new();
    left_map.insert("Rust", 2010);
    left_map.insert("Go", 2009);
    let mut right_map: HashMap<&str, i32> = HashMap::new();
    right_map.insert("Zig", 2016);
    right_map.insert("Rust", 2015);
    let merged = merge_maps(&left_map, &right_map);
    println!("合并后的语言年份: {:?}", merged);
    println!("字符串列表: {:?}", to_string_vec(&numbers));
    println!("排序后的键: {:?}", sorted_keys(&merged));
    println!(
        "最大数字: {:?}",
        max_by_key(&numbers, |value| *value)
    );
    let tags = vec![
        ("Rust", "systems"),
        ("Rust", "web"),
        ("Go", "cloud"),
        ("Rust", "embedded"),
    ];
    println!("按语言分组: {:?}", group_values(&tags));

    println!("\n== 循环练习 ==");
    println!("倒计时: {:?}", countdown(5));
    println!("奇数平方: {:?}", odd_squares(1, 10));
    println!("乘法表 1..=3: {:?}", multiplication_table(3));
    println!("斐波那契(7): {:?}", fibonacci_sequence(7));
    println!("帕斯卡(4): {:?}", pascal_triangle(4));
    println!("5! = {}", factorial(5));
    println!("前 5 个三角数: {:?}", triangle_numbers(5));

    println!("\n== 生命周期练习 ==");
    let sentence = "Rust ownership keeps concurrency fearless";
    println!("较长字符串: {}", pick_longest("Rust", "Ferris"));
    println!("最短单词: {:?}", shortest_word(sentence));
    let noted = longest_with_note("borrow checker", "ownership rules", "比较两个术语长度...");
    println!("提示后得到较长的词组: {}", noted);
    println!("最长单词: {:?}", longest_word(sentence));
    let article = Article::new(
        "Rust makes systems programming fun",
        "Ownership and borrowing keep things safe.",
    );
    println!("标题预览: {}", article.teaser());
    println!("正文长度: {}", article.body_length());
    let keywords = ["async", "ownership", "lifetime", "borrow"];
    println!(
        "包含 ship 的词: {:?}",
        find_match(&keywords, |word| word.ends_with("ship"))
    );

    println!("\n== 结构体与方法 ==");
    let mut roster = vec![Player::new("Ferris"), Player::new("Rustacean"), Player::new("Alice")];
    roster[0].add_score(15);
    roster[0].apply_penalty(3);
    roster[1].add_score(20);
    roster[2].add_score(9);
    if let Some(best) = highest_scorer(&roster) {
        println!("最高分玩家: {} ({})", best.name, best.score);
    }
    if let Some(alice) = find_player(&roster, "Alice") {
        println!("找到玩家 Alice，目前积分 {}", alice.score);
    }
    let rewarded = award_bonus(&mut roster, 10, 5);
    println!("发放奖励人数: {}", rewarded);
    if let Some(best) = highest_scorer(&roster) {
        println!("奖励后最高分玩家: {} ({})", best.name, best.score);
    }
    println!("平均积分: {:?}", average_score(&roster));
    let top_two_names: Vec<_> = top_n_players(&roster, 2)
        .into_iter()
        .map(|player| player.name.as_str())
        .collect();
    println!("积分前两名: {:?}", top_two_names);

    println!("\n== 枚举与命令解析 ==");
    let first = parse_command("inc 42").expect("合法命令");
    println!("命令解析: {:?}, 描述: {}", first, describe_command(&first));
    let program = vec![
        first,
        Command::Decrement(10),
        Command::Reset,
        Command::Increment(5),
    ];
    println!("执行命令后的结果: {}", run_program(0, &program));
    println!("命令文本:\n{}", serialize_program(&program));
    println!("执行历史: {:?}", execute_with_history(0, &program));
    println!("优化后的命令: {:?}", optimize_program(&program));
    println!(
        "限定范围执行结果: {}",
        execute_with_limits(0, &program, -5, 10)
    );

    println!("\n== 集合与错误处理 ==");
    let records = [("Alice", 90), ("Bob", 75), ("Alice", 95), ("Bob", 85)];
    println!("总分: {:?}", total_scores(&records));
    println!("平均分: {:?}", student_average(&records));
    println!("最佳学生: {:?}", top_student(&records));
    println!("及格情况: {:?}", pass_fail(&records, 80.0));
    println!("成绩中位数: {:?}", class_median(&records));
    println!("成绩分布: {:?}", grade_distribution(&records));
    println!(
        "成绩占比: {:?}",
        normalize_scores(&records).expect("总分应大于零")
    );

    println!("\n== 迭代器与闭包 ==");
    let nums = [1, -3, 2, 4, 2, 6, 6];
    println!("正数个数: {}", count_positive(&nums));
    println!("偶数平方和: {}", even_square_sum(&nums));
    println!("唯一偶数: {:?}", unique_sorted_evens(&nums));
    println!("前缀和: {:?}", running_total(&nums));
    println!("按符号分组: {:?}", partition_by_sign(&nums));
    println!("笛卡尔积: {:?}", cartesian_pairs(&[1, 2], &[3, 4]));
    println!("第一个重复值: {:?}", first_duplicate(&nums));

    println!("\n== 并发练习 ==");
    let sum_handle = spawn_sum(vec![1, 2, 3, 4, 5]);
    println!("线程求和: {}", sum_handle.join().unwrap());
    println!("并行求和: {}", parallel_sum(&[1, 2, 3, 4, 5, 6, 7, 8]));
    let messages = vec![
        "hello".to_string(),
        "from".to_string(),
        "threads".to_string(),
    ];
    println!("消息收集: {:?}", collect_messages(&messages));
    println!("并发计数: {}", concurrent_counter(4, 1_000));
    let task_results = run_tasks(vec![
        Box::new(|| 1 + 1),
        Box::new(|| 21 * 2),
        Box::new(|| 7 * 7),
    ]);
    println!("任务结果: {:?}", task_results);
    println!(
        "按块求和: {}",
        chunked_sum(vec![1, 2, 3, 4, 5, 6], 2)
    );
    let handles: Vec<_> = (0..3)
        .map(|_| thread::spawn(|| ()))
        .collect();
    println!("完成的线程数: {}", wait_for_all(handles));
}
