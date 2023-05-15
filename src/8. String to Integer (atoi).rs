impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut index = 0;
        let mut num = 0;
        // remove space
        while index < s.len() && s[index] == ' ' {
            index += 1;
        }
        if s.len() == index {
            return 0;
        }
        // sign
        let mut sign = 1;
        if s[index] == '+' {
            index += 1;
        } else if s[index] == '-' {
            index += 1;
            sign = -1;
        }
        while index < s.len() {
            if s[index] < '0' || s[index] > '9' {
                break;
            }
            let curr_num: i32 = s[index] as i32 - '0' as i32;
            if num > i32::MAX / 10 || (num == i32::MAX / 10 && curr_num > i32::MAX % 10) {
                return i32::MAX;
            }
            if num < i32::MIN / 10 || (num == i32::MIN / 10 && curr_num > -(i32::MIN % 10)) {
                return i32::MIN;
            }
            num = num * 10 + curr_num * sign;
            index += 1;
        }
        num
    }
}