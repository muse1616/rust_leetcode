impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut index = 0;
        for i in 0..nums.len() {
            if index < 2 || nums[i] > nums[index - 2] {
                nums[index] = nums[i];
                index += 1;
            }
        }

        index as i32
    }
}