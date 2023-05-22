impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s = s.to_lowercase();
        let mut s = s.chars().collect::<Vec<_>>();

        let mut i = 0 as i32;
        let mut j = s.len() as i32 - 1;
        while i < j {
            while i < j && !s[i as usize].is_alphanumeric() {
                i += 1;
            }
            while i < j && !s[j as usize].is_alphanumeric() {
                j -= 1;
            }
            if s[i as usize] != s[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}
