impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut i = (n - 2) as i32;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }
        if i >= 0 {
            let mut j = n as i32 - 1;
            while j >= 0 && nums[i as usize] >= nums[j as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j as usize);
        }

        nums[(i + 1) as usize..n].reverse();
    }
}