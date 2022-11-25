// leetcode-1480
struct Solution{}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect()
    }
}

fn main() {
    let solution = Solution::running_sum(vec![1, 2, 3, 4]);
    println!("{:?}", solution);
}
