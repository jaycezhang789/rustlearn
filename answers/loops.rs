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

pub fn fibonacci_sequence(n: u32) -> Vec<u64> {
    match n {
        0 => Vec::new(),
        1 => vec![0],
        _ => {
            let mut seq = Vec::with_capacity(n as usize);
            seq.push(0);
            seq.push(1);
            while seq.len() < n as usize {
                let len = seq.len();
                let next = seq[len - 1] + seq[len - 2];
                seq.push(next);
            }
            seq
        }
    }
}

pub fn pascal_triangle(rows: usize) -> Vec<Vec<u64>> {
    let mut triangle: Vec<Vec<u64>> = Vec::with_capacity(rows);
    for i in 0..rows {
        let mut row = vec![1u64; i + 1];
        for j in 1..i {
            row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
        }
        triangle.push(row);
    }
    triangle
}

pub fn factorial(n: u32) -> u128 {
    (1..=n as u128).product::<u128>().max(1)
}

pub fn triangle_numbers(count: usize) -> Vec<u64> {
    let mut results = Vec::with_capacity(count);
    let mut current = 0u64;
    for idx in 1..=count {
        current += idx as u64;
        results.push(current);
    }
    results
}
