impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::new();
        for (index, v) in nums.iter().enumerate() {
            if map.contains_key(v) && (index as i32 - map[v] as i32).abs() <= k {
                return true;
            }
            *map.entry(v).or_insert(index) = index;
        }
        false
    }
}