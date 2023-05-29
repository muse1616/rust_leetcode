impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut ans = 0;
        let mut i = 0;
        while i < 32 && x > 0 {
            ans |= (x & 1) << (31 - i);
            x >>= 1;
            i += 1;
        }
        ans
    }
}