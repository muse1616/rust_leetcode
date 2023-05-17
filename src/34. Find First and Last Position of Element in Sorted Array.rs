impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![-1, -1];
        let n = nums.len();
        let mut left : i32 = 0;
        let mut right  : i32 =  n as i32- 1;
        while left <= right {
            let middle = left + (right - left) / 2;
            if nums[middle as usize] == target {
                ans[0] = middle;
                right = middle - 1;
            } else if nums[middle as usize] > target {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }

        let mut left : i32 = 0;
        let mut right  : i32 =  n as i32- 1;
        while left <= right {
            let middle = left + (right - left) / 2;
            if nums[middle as usize] == target {
                ans[1] = middle;
                left = middle + 1;
            } else if nums[middle as usize] > target {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }

        ans
    }
}