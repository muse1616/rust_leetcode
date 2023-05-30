// impl Solution {
//     fn get_id(x: i32, w: i32) -> i32 {
//         if x < 0 {
//             (x + 1) / w - 1
//         } else {
//             x / w
//         }
//     }
//     pub fn contains_nearby_almost_duplicate(
//         nums: Vec<i32>,
//         index_diff: i32,
//         value_diff: i32,
//     ) -> bool {
//         let mut bucket: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
//         let n = nums.len();
//         for i in 0..n {
//             let x = nums[i];
//             let id = Self::get_id(x, value_diff + 1);
//             if bucket.contains_key(&id) {
//                 return true;
//             }
//             if bucket.contains_key(&(id - 1)) && ((x - bucket[&(id - 1)]).abs() <= index_diff) {
//                 return true;
//             }

//             if bucket.contains_key(&(id + 1)) && ((x - bucket[&(id + 1)]).abs() <= index_diff) {
//                 return true;
//             }
//             bucket.insert(id, x);
//             if i as i32 >= index_diff {
//                 bucket.remove(&Self::get_id(
//                     nums[(i as i32 - index_diff) as usize],
//                     value_diff + 1,
//                 ));
//             }
//         }
//         false
//     }
// }
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, mut k: i32, t: i32) -> bool {
        if  nums.len() == 0 {
            return false;
        }
        let mut k: usize = k as usize;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..(k + 1) {
            if i >= nums.len() {
                break;
            }
            let key = get_id(nums[i], t);
            if map.contains_key(&key) {
                return true;
            } else if map.contains_key(&(key - 1)) && i64::abs(map[&(key - 1)] as i64 - nums[i] as i64) <= t as i64{
                return true;
            } else if map.contains_key(&(key + 1)) && i64::abs(map[&(key + 1)] as i64 - nums[i] as i64) <= t as i64{
                return true;
            }
            map.insert(get_id(nums[i], t), nums[i]);
        }
        for i in (k + 1)..nums.len() {
            let key = get_id(nums[i], t);
            map.remove(&get_id(nums[i - 1 - k], t));
            if map.contains_key(&key) {
                return true;
            } else if map.contains_key(&(key - 1)) && i64::abs(map[&(key - 1)] as i64 - nums[i] as i64) <= t as i64 {
                return true;
            } else if map.contains_key(&(key + 1)) && i64::abs(map[&(key + 1)] as i64 - nums[i] as i64) <= t as i64 {
                return true;
            }
            map.insert(get_id(nums[i], t), nums[i]);
        }

        false
    }
}

pub fn get_id(num: i32, t: i32) -> i32 {
    if num >= 0 {
        return num / (t + 1);
    } else {
        return (num + 1) / (t + 1) - 1;
    }
    
}