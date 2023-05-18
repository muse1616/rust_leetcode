impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();

        strs.into_iter().for_each(|str| {
            let mut cnt = [0; 26];
            str.bytes().for_each(|c| {
                cnt[(c - b'a') as usize] += 1;
            });
            map.entry(cnt).or_insert(vec![]).push(str);
        });

        map.into_values().collect()
    }
}