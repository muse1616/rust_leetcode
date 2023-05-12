impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        let mut index = m + n - 1;
        while n > 0 {
            let mut num = 0;
            if m > 0 && n > 0 {
                if nums1[m as usize - 1] > nums2[n as usize - 1] {
                    num = nums1[m as usize - 1];
                    m -= 1;
                } else {
                    num = nums2[n as usize - 1];
                    n -= 1;
                }
            } else {
                num = nums2[n as usize - 1];
                n -= 1;
            }
            nums1[index as usize] = num;
            index -= 1;
        }
    }
}