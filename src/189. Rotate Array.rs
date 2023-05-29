impl Solution {
    fn reverse(nums: &mut Vec<i32>, mut i: i32, mut j: i32) {
        while i < j {
            let tmp = nums[i as usize];
            nums[i as usize] = nums[j as usize];
            nums[j as usize] = tmp;
            i += 1;
            j -= 1;
        }
    }
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k % nums.len() as i32;
        Self::reverse(nums, 0, nums.len() as i32 - 1);
        Self::reverse(nums, 0, k - 1);
        Self::reverse(nums, k, nums.len() as i32 - 1);
    }
}