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
