impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        // normal
        // let a = a.as_bytes();
        // let b = b.as_bytes();
        // let mut carry = 0;
        // let mut ans = vec![];
        // let mut i = (a.len() - 1) as i32;
        // let mut j = (b.len() - 1) as i32;
        // while i >= 0 || j >= 0 || carry != 0 {
        //     if i >= 0 {
        //         carry += (a[i as usize] - b'0');
        //         i -= 1;
        //     }
        //     if j >= 0 {
        //         carry += (b[j as usize] - b'0');
        //         j -= 1;
        //     }
        //     ans.push(char::from(carry % 2 + 48));
        //     carry /= 2;
        // }

        // ans.iter().rev().collect()
        let max = std::cmp::max(a.len(), b.len());
        if a.len() < max {
            let v = vec![48; max - a.len()];
            a = format!("{}{}", String::from_utf8(v).unwrap(), a);
        }
        if b.len() < max {
            let v = vec![48; max - b.len()];
            b = format!("{}{}", String::from_utf8(v).unwrap(), b);
        }
        let mut carry = 0;
        let mut m = a
            .as_bytes()
            .iter()
            .rev()
            .zip(b.as_bytes().iter().rev())
            .map(|x| {
                carry += x.0 - 48;
                carry += x.1 - 48;
                let n = carry % 2 + 48;
                carry /= 2;
                n
            })
            .collect::<Vec<u8>>();
        while carry > 0 {
            let n = carry % 2 + 48;
            m.push(n);
            carry /= 2;
        }
        m.reverse();
        String::from_utf8(m).unwrap()
    }
}