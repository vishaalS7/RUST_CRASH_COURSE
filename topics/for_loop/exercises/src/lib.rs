pub fn sum(nums: Vec<i32>) -> i32 {
    // return the sum of all elements in the vector
    nums.iter().sum()
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    vec![i; n]
}
