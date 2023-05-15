impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let mut res: i64 = 0;
        let mut m = (dividend as i64).abs();
        let mut n = (divisor as i64).abs();
        if m < n {
            return 0;
        }
        let mut t = n;
        let mut p: i64 = 1;
        while m > (t << 1) {
            t <<= 1;
            p <<= 1;
        }
        res += p + (Self::divide((m - t) as i32, n as i32)) as i64;

        if (dividend < 0) ^ (divisor < 0) {
            res = -res;
        }
        if res > i32::MAX as i64 {
            return i32::MAX;
        }
        res as i32
    }
}