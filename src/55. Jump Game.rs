impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max = 0;
        let n = nums.len() as i32;
        for i in 0..n{
            if i <= max{
                max = max.max(nums[i as usize] + i);
                if max >= n - 1{
                    return true;
                }
            }
        }
        false
    }
}