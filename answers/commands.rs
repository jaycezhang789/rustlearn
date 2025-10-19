#[derive(Debug, PartialEq)]
pub enum Command {
    Increment(i32),
    Decrement(i32),
    Reset,
}

pub fn describe_command(cmd: &Command) -> String {
    match cmd {
        Command::Increment(value) => format!("增加 {}", value),
        Command::Decrement(value) => format!("减少 {}", value),
        Command::Reset => "重置".to_string(),
    }
}

pub fn parse_command(input: &str) -> Option<Command> {
    let mut parts = input.split_whitespace();
    match parts.next()? {
        "inc" => {
            let value = parts.next()?.parse().ok()?;
            if parts.next().is_none() {
                Some(Command::Increment(value))
            } else {
                None
            }
        }
        "dec" => {
            let value = parts.next()?.parse().ok()?;
            if parts.next().is_none() {
                Some(Command::Decrement(value))
            } else {
                None
            }
        }
        "reset" => {
            if parts.next().is_none() {
                Some(Command::Reset)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn run_program(start: i32, commands: &[Command]) -> i32 {
    let mut value = start;
    for command in commands {
        match command {
            Command::Increment(delta) => value += delta,
            Command::Decrement(delta) => value -= delta,
            Command::Reset => value = 0,
        }
    }
    value
}

pub fn serialize_program(commands: &[Command]) -> String {
    let lines: Vec<String> = commands
        .iter()
        .map(|command| match command {
            Command::Increment(delta) => format!("inc {}", delta),
            Command::Decrement(delta) => format!("dec {}", delta),
            Command::Reset => "reset".to_string(),
        })
        .collect();
    lines.join("\n")
}

pub fn execute_with_history(start: i32, commands: &[Command]) -> Vec<i32> {
    let mut history = Vec::with_capacity(commands.len() + 1);
    let mut value = start;
    history.push(value);
    for command in commands {
        match command {
            Command::Increment(delta) => value += delta,
            Command::Decrement(delta) => value -= delta,
            Command::Reset => value = 0,
        }
        history.push(value);
    }
    history
}

pub fn optimize_program(commands: &[Command]) -> Vec<Command> {
    let mut optimized = Vec::new();
    let mut pending_type: Option<bool> = None; // true => Increment, false => Decrement
    let mut pending_value: i32 = 0;

    let flush = |optimized: &mut Vec<Command>,
                 pending_type: &mut Option<bool>,
                 pending_value: &mut i32| {
        if let Some(kind) = pending_type.take() {
            if *pending_value != 0 {
                let value = *pending_value;
                optimized.push(if kind {
                    Command::Increment(value)
                } else {
                    Command::Decrement(value)
                });
            }
            *pending_value = 0;
        }
    };

    for command in commands {
        match command {
            Command::Increment(delta) => {
                match pending_type {
                    Some(true) => pending_value += delta,
                    Some(false) => {
                        flush(&mut optimized, &mut pending_type, &mut pending_value);
                        pending_type = Some(true);
                        pending_value = *delta;
                    }
                    None => {
                        pending_type = Some(true);
                        pending_value = *delta;
                    }
                }
            }
            Command::Decrement(delta) => match pending_type {
                Some(false) => pending_value += delta,
                Some(true) => {
                    flush(&mut optimized, &mut pending_type, &mut pending_value);
                    pending_type = Some(false);
                    pending_value = *delta;
                }
                None => {
                    pending_type = Some(false);
                    pending_value = *delta;
                }
            },
            Command::Reset => {
                flush(&mut optimized, &mut pending_type, &mut pending_value);
                optimized.push(Command::Reset);
            }
        }
    }

    flush(&mut optimized, &mut pending_type, &mut pending_value);
    optimized
}

pub fn execute_with_limits(start: i32, commands: &[Command], min: i32, max: i32) -> i32 {
    let mut value = start;
    for command in commands {
        match command {
            Command::Increment(delta) => value += delta,
            Command::Decrement(delta) => value -= delta,
            Command::Reset => value = 0,
        }
        if value < min || value > max {
            return value;
        }
    }
    value
}
