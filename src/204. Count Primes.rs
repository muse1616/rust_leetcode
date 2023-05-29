impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as i64;
        let mut is_prime = vec![1; n as usize];
        let mut ans = 0;
        for i in 2..n {
            if is_prime[i as usize] == 1 {
                ans += 1;
                if i * i < n {
                    for j in ((i * i)..n).step_by(i as usize) {
                        is_prime[j as usize] = 0;
                    }
                }
            }
        }
        ans
    }
}