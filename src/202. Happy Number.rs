impl Solution {
    fn get_next(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            sum += d * d;
        }
        sum
    }
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Self::get_next(n);
        while fast != 1 && slow != fast {
            slow = Self::get_next(slow);
            fast = Self::get_next(Self::get_next(fast));
        }
        fast == 1
    }
}
