impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let mut sum = Vec::new();

        let mut i = num1.len() as i32 - 1;
        let mut j = num2.len() as i32 - 1;
        let mut carry = 0;
        while i >= 0 || j >= 0 || carry != 0 {
            let n1 = {
                if i >= 0 {
                    num1[i as usize] - '0' as u8
                } else {
                    0
                }
            };
            let n2 = {
                if j >= 0 {
                    num2[j as usize] - '0' as u8
                } else {
                    0
                }
            };
            carry = carry + n1 + n2;
            sum.push(carry % 10 + '0' as u8);
            carry /= 10;
            i-=1;
            j-=1;
        }
        sum.reverse();
        String::from_utf8(sum).unwrap()
    }
}