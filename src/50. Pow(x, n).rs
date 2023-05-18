impl Solution {
    fn quick(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        } else if n % 2 == 1 {
            return Self::my_pow(x, (n - 1) as i32) * x;
        }
        let tmp = Self::my_pow(x, n as i32 / 2);
        tmp * tmp
    }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // let n = n as i64;
        // if n >= 0 {
        //     Self::quick(x, n)
        // } else {
        //     1.0 / Self::quick(x, -n)
        // }
        let mut res: f64 = 1.0;
        let mut m = n;
        let mut xx = x;
        loop {
            if m == 0 {
                break;
            }
            if m % 2 != 0 {
                res *= xx;
            }
            xx *= xx;
            m /= 2;
        }
        if n < 0 {
            return 1.0 / res;
        }
        res
    }
}