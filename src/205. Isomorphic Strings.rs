impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        
        let mut s2t = HashMap::new();
        let mut t2s = HashMap::new();
        let n = s.len();
        let s = s.as_bytes();
        let t = t.as_bytes();
        for i in 0..n{
            let x = s[i];
            let y = t[i];
            if (s2t.contains_key(&x) && s2t[&x]!=y) ||(t2s.contains_key(&y) && t2s[&y]!=x){
                return false;
            }
            s2t.insert(x, y);
            t2s.insert(y, x);
        }
        true
    }
}