impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut number = 0;
        let mut multiple = 1;
        let column_title = column_title.as_bytes();
        column_title.iter().rev().for_each(|x| {
            let k = x - b'A' + 1;
            number += k as i32 * multiple;
            multiple *= 26;
        });
        number
    }
}