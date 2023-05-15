impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans = vec![];
        let n = nums.len();
        for first in 0..n {
            if first > 0 && nums[first - 1] == nums[first] {
                continue;
            }
            let mut third = n - 1;
            for second in (first + 1)..n {
                if second > first + 1 && nums[second] == nums[second - 1] {
                    continue;
                }
                while second < third && nums[first] + nums[second] + nums[third] > 0 {
                    third -= 1;
                }
                if second == third {
                    break;
                }
                if nums[first] + nums[second] + nums[third] == 0 {
                    ans.push(vec![nums[first], nums[second], nums[third]]);
                }
            }
        }
        ans
    }
}