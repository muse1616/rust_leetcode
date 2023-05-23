impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;
        while low < high {
            let middle = low + (high - low) / 2;
            if nums[middle as usize] < nums[high as usize] {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        nums[low as usize]
    }
}
