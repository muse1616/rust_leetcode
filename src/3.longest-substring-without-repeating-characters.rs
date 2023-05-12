impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut last : [i32;128]= [-1;128];
        let mut len = 0;
        let mut left = -1;
        for (i,v) in s.chars().enumerate(){
            left = max(left,last[v as usize] as i32);
            last[v as usize] = i as i32;
            len = std::cmp::max(len, i as i32 - left)
        }
        len

    }
}

