impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];
        if nums.len() == 1 {
            return nums[0];
        }
        dp[0] = nums[0];
        dp[1] = dp[0].max(nums[1]);
        for i in 2..n {
            dp[i] = (dp[i - 2] + nums[i]).max(dp[i - 1]);
        }
        dp[n - 1]
    }
}