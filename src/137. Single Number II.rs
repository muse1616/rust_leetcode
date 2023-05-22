impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            let mut total = 0;
            for &n in nums.iter() {
                total += (n >> i & 1);
            }
            if total % 3 == 1 {
                ans |= 1 << i;
            }
        }
        ans
    }
}