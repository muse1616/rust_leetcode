impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashMap;
        let mut ans = vec![];
        if s.len() <= 10{
            return ans;
        }
        let mut map = HashMap::new();
        let mut i = 0;
        while i <= s.len() - 10 {
            let sub = &s[i..i + 10];
            *map.entry(sub).or_insert(0) += 1;
            if map[sub] == 2 {
                ans.push(String::from(sub));
            }
            i+=1;
        }
        ans
    }
}