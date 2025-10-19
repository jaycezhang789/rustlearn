use std::collections::HashSet;

pub fn count_positive(nums: &[i32]) -> usize {
    nums.iter().filter(|&&n| n > 0).count()
}

pub fn even_square_sum(nums: &[i32]) -> i32 {
    nums.iter()
        .filter(|n| *n % 2 == 0)
        .map(|n| n * n)
        .sum()
}

pub fn unique_sorted_evens(nums: &[i32]) -> Vec<i32> {
    let mut evens: Vec<i32> = nums.iter().copied().filter(|n| n % 2 == 0).collect();
    evens.sort();
    evens.dedup();
    evens
}

pub fn running_total(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |state, &value| {
            *state += value;
            Some(*state)
        })
        .collect()
}

pub fn partition_by_sign(nums: &[i32]) -> (Vec<i32>, Vec<i32>) {
    nums.iter().copied().partition(|n| *n > 0)
}

pub fn cartesian_pairs(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    let mut pairs = Vec::with_capacity(a.len() * b.len());
    for &left in a {
        for &right in b {
            pairs.push((left, right));
        }
    }
    pairs
}

pub fn first_duplicate(nums: &[i32]) -> Option<i32> {
    let mut seen = HashSet::new();
    for &value in nums {
        if !seen.insert(value) {
            return Some(value);
        }
    }
    None
}
