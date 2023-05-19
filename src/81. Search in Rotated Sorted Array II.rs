impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len();
        let mut left = 0 as i32;
        let mut right = n as i32 - 1;
        while left <= right {
            let middle = left + (right - left) / 2;
            if nums[middle as usize] == target {
                return true;
            }
            if nums[left as usize] == nums[middle as usize]
                && nums[middle as usize] == nums[right as usize]
            {
                left += 1;
                right -= 1;
            } else if nums[left as usize] <= nums[middle as usize] {
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
        false
    }
}
