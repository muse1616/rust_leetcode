impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for &n in nums.iter() {
            set.insert(n);
        }
        let mut ans = 0;
        for &n in nums.iter() {
            if !set.contains(&(n - 1)) {
                let mut current_len = 0;
                let mut current_num = n;
                while set.contains(&current_num) {
                    current_len += 1;
                    current_num += 1;
                }
                ans = ans.max(current_len);
            }
        }

        ans
    }
}
