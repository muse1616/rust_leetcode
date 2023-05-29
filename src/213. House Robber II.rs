impl Solution {
    pub fn rob_range(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let mut first = nums[start];
        let mut second = first.max(nums[start + 1]);
        for i in (start + 2)..=end {
            let tmp = second;
            second = second.max(first + nums[i]);
            first = tmp;
        }
        second
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            nums[0]
        } else if n == 2 {
            nums[0].max(nums[1])
        } else {
            Self::rob_range(&nums, 0, n - 2).max(Self::rob_range(&nums, 1, n - 1))
        }
    }
}