impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        let n = haystack.len();
        let m = needle.len();
        if m > n {
            return -1;
        }
        for i in 0..=n - m {
            let mut flag = true;
            for j in 0..m {
                if haystack[i + j] != needle[j] {
                    flag = false;
                    break;
                }
            }
            if flag {
                return i as i32;
            }
        }
        -1
    }
}