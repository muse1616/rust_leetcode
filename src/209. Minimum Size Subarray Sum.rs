impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut start = 0;
        let mut end = 0;
        let mut ans = i32::MAX;
        let mut sum = 0;
        while end < n {
            sum += nums[end];
            while sum >= target {
                ans = ans.min(end as i32 - start as i32 + 1);
                sum -= nums[start];
                start += 1;
            }
            end += 1;
        }

        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }
}