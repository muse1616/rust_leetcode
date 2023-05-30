impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for n in nums {
            if set.contains(&n) {
                return true;
            }
            set.insert(n);
        }
        false
    }
}