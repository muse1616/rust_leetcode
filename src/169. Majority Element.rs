impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = -1;
        let mut count = 0;
        nums.iter().for_each(|&n| {
            if candidate == n {
                count += 1;
            } else {
                count -= 1;
                if count < 0 {
                    candidate = n;
                    count = 1;
                }
            }
        });

        candidate
    }
}