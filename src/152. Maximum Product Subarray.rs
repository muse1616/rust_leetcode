impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut max_p = nums[0];
        let mut min_p = nums[0];

        for &n in nums.iter().skip(1) {
            let mut mx = max_p;
            let mut mn = min_p;
            max_p = n.max((n * mx).max(mn * n));
            min_p = n.min((n * mx).min(mn * n));
            ans = ans.max(max_p);
        }

        ans
    }
}