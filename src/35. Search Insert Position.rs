impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // normal
        // let (mut left ,mut right) = (0,nums.len()-1);
        // let mut ans = 0;
        // while left <= right{
        //     let middle = left + (right-left)/2;
        //     if nums[middle] == target{
        //         return middle as i32;
        //     }else if nums[middle] < target{
        //         left = middle + 1;
        //     }else{
        //         if middle == 0{
        //             return 0;
        //         }
        //         right = middle-1;
        //     }
        // }
        // left as i32

        // functional stytle
        // nums.binary_search(&target).map_or_else(|e|e,|v|v) as i32
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}