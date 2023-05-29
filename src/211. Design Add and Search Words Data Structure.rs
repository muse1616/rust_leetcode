struct DictTree {
    is_end: bool,
    son: Vec<Option<DictTree>>,
}

impl DictTree {
    fn new(is_end: bool) -> Self {
        let mut son = Vec::with_capacity(26);
        (0..26).for_each(|_| son.push(None));
        Self {is_end, son}
    }
}

struct WordDictionary {
    tab: DictTree,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Self {tab: DictTree::new(false)}
    }
    
    fn add_word(&mut self, word: String) {
        fn dfs(dt: &mut DictTree, s: &[u8], i: usize) {
            if i < s.len() {
                let c = s[i] as usize - b'a' as usize;
                let t = if let Some(t) = dt.son[c].as_mut() {t}
                else {dt.son[c] = Some(DictTree::new(false)); dt.son[c].as_mut().unwrap()};
                dfs(t, s, i + 1)
            } else {dt.is_end = true}
        }
        dfs(&mut self.tab, &mut word.as_bytes(), 0);
    }
    
    fn search(&self, word: String) -> bool {
        fn dfs(dt: &DictTree, s: &[u8], i: usize) -> bool {
            if i < s.len() {
                if s[i] == b'.' {
                    for son in dt.son.iter() {
                        if let Some(t) = son {
                            if dfs(t, s, i + 1) {return true}
                        }
                    }
                } else {
                    let c = s[i] as usize - b'a' as usize;
                    if let Some(t) = dt.son[c].as_ref() {
                        return dfs(t, s, i + 1);
                    }
                }
                false
            } else {
                if dt.is_end {true} else {false}
            }
        }
        dfs(&self.tab, &word.as_bytes(), 0)
    }
}