impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s = s
            .split(" ")
            .filter(|&x| x != "" && x != " ")
            .collect::<Vec<_>>();
        s.reverse();
        s.join(" ")
    }
}