impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut closest = i32::MAX;
        let n: usize = nums.len();
        for first in 0..n {
            if first > 0 && nums[first - 1] == nums[first] {
                continue;
            }
            let mut second = first + 1;
            let mut third = n - 1;
            while second < third {
                let sum = nums[first] + nums[second] + nums[third];
                if sum == target {
                    return target;
                }
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
                if sum > target {
                    let mut i = third - 1;
                    while second < i && nums[i] == nums[third] {
                        i -= 1;
                    }
                    third = i;
                } else {
                    let mut i = second + 1;
                    while i < third && nums[i] == nums[second] {
                        i += 1;
                    }
                    second = i;
                }
            }
        }
        closest
    }
}