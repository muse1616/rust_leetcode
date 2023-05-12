impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.bytes()
            .rev()
            .fold((0, 0), |sum, cur| {
                let n = match cur {
                    b'I' => 1,
                    b'V' => 5,
                    b'X' => 10,
                    b'L' => 50,
                    b'C' => 100,
                    b'D' => 500,
                    b'M' => 1000,
                    _ => -9999,
                };
                (if n < sum.1 { sum.0 - n } else { sum.0 + n }, n)
            })
            .0
    }
}
