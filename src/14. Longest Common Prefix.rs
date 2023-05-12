impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // let mut cur = 0;
        // for i in 0..strs[0].len() {
        //     for s in strs.iter() {
        //         let s = s.as_bytes();
        //         if s.len() == i || strs[0].as_bytes()[i] != s[i] {
        //             return strs[0][0..cur].to_string();
        //         }
        //     }
        //     cur += 1;
        // }
        // strs[0][0..cur].to_string()

        // iter style
        strs.iter()
            .max()
            .unwrap()
            .chars()
            .zip(strs.iter().min().unwrap().chars())
            .take_while(|x: &(char, char)| x.0 == x.1)
            .map(|x| x.0)
            .collect()
    }
}