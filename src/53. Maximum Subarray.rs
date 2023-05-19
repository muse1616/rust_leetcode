
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut curr = 0;
        for n in nums.iter() {
            curr = (curr + n).max(*n);
            max = max.max(curr);
        }
        max
    }
}