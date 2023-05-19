impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut p0 = 0;
        let mut p2 = n-1;

        for i in 0..=p2 {
            while i <= p2 && nums[i] == 2 {
                nums.swap(i, p2);
                if p2 == 0{
                    return;
                }
                p2 -= 1;
            }
            if nums[i] == 0 {
                nums.swap(i, p0);
                p0 += 1;
            }
        }
    }
}
