impl Solution {
    fn dfs(s: &[u8], seg_id: usize, seg_start: usize, segment: &mut Vec<i32>, ans: &mut Vec<String>) {
        if seg_id == 4{
            if seg_start == s.len(){
                ans.push(segment.to_vec().iter().map(|n|n.to_string()).collect::<Vec<_>>().join("."))
            }
            return;
        }
        if seg_start == s.len(){
            return;
        }
        if s[seg_start] == b'0'{
            segment[seg_id] = 0;
            Self::dfs(s, seg_id + 1, seg_start + 1, segment, ans);
            return;
        }
        let mut address = 0;
        for seg_end in seg_start..s.len(){
            address = address * 10 + (s[seg_end] as i32 - '0' as i32);
            if address >  0 && address <= 0xff{
                segment[seg_id] = address;
                Self::dfs(s, seg_id+1, seg_end+1, segment, ans)
            }else{
                break;
            }
        }

    }
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut ans = vec![];
        let mut segment = vec![0; 4];
        Self::dfs(s, 0, 0, &mut segment, &mut ans);
        ans
    }
}