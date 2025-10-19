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
