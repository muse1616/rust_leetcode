impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let mut rightmost = 0;
        let mut end = 0;
        let n = nums.len() as i32;
        for i in 0..n - 1 {
            rightmost = rightmost.max(nums[i as usize] + i);
            if i == end {
                end = rightmost;
                steps += 1;
            }
        }
        steps
    }
}