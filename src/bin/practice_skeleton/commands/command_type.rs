#[derive(Debug, PartialEq)]
pub enum Command {
    Increment(i32),
    Decrement(i32),
    Reset,
}
