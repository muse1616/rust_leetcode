impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut left: i32 = 0;
        let mut right: i32 = n as i32 - 1;
        while left <= right {
            let middle = left + (right - left) / 2;
            if nums[middle as usize] == target {
                return middle;
            }
            if nums[left as usize] <= nums[middle as usize] {
                if nums[left as usize] <= target && nums[middle as usize] > target {
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            } else {
                if nums[middle as usize] < target && nums[right as usize] >= target {
                    left = middle + 1;
                } else {
                    right = middle - 1;
                }
            }
        }
        -1
    }
}