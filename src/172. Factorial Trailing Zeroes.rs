impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0{
            n /= 5;
            ans += n;
        }
        ans
    }
}