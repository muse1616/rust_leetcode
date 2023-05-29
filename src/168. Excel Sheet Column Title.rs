impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut ans = vec![];
        while column_number > 0 {
            column_number -= 1;
            let c = (column_number % 26) as u8 + b'A';
            ans.push(c as char);
            column_number /= 26;
        }
        ans.iter().rev().collect()
    }
}