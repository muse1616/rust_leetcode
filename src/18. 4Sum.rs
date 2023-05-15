impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort();
        let len = nums.len();
        for k in 0..len {
            // 剪枝
            if nums[k] > target && (nums[k] > 0 || target > 0) {
                break;
            }
            // 去重
            if k > 0 && nums[k] == nums[k - 1] {
                continue;
            }
            for i in (k + 1)..len {
                // 剪枝
                if nums[k] + nums[i] > target && (nums[k] + nums[i] >= 0 || target >= 0) {
                    break;
                }
                // 去重
                if i > k + 1 && nums[i] == nums[i - 1] {
                    continue;
                }
                let (mut left, mut right) = (i + 1, len - 1);
                while left < right {
                    if nums[k] + nums[i] > target - (nums[left] + nums[right]) {
                        right -= 1;
                        // 去重
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    } else if nums[k] + nums[i] < target - (nums[left] + nums[right]) {
                        left += 1;
                        // 去重
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                    } else {
                        result.push(vec![nums[k], nums[i], nums[left], nums[right]]);
                        // 去重
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }
        result
    }
}