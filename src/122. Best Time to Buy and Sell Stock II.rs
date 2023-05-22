impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp0 = 0;
        let mut dp1 = -prices[0];
        for i in 1..prices.len() {
            dp0 = dp0.max(prices[i] + dp1);
            dp1 = dp1.max(dp0 - prices[i]);
        }
        dp0
    }
}