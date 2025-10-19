pub fn countdown(start: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut current = start;
    loop {
        result.push(current);
        if current == 0 {
            break;
        }
        current -= 1;
    }
    result
}

pub fn odd_squares(start: i32, end: i32) -> Vec<i32> {
    if start >= end {
        return Vec::new();
    }
    (start..end)
        .filter(|n| n % 2 != 0)
        .map(|n| n * n)
        .collect()
}

pub fn multiplication_table(size: u32) -> Vec<Vec<u32>> {
    let mut table = Vec::new();
    for i in 1..=size {
        let mut row = Vec::new();
        for j in 1..=size {
            row.push(i * j);
        }
        table.push(row);
    }
    table
}
