impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        // let mut dic = vec![
        //     (1000, "M"),
        //     (900, "CM"),
        //     (500, "D"),
        //     (400, "CD"),
        //     (100, "C"),
        //     (90, "XC"),
        //     (50, "L"),
        //     (40, "XL"),
        //     (10, "X"),
        //     (9, "IX"),
        //     (5, "V"),
        //     (4, "IV"),
        //     (1, "I"),
        // ];
        // let mut ans = String::new();
        // for (n, c) in dic {
        //     while num >= n {
        //         num -= n;
        //         ans.push_str(c);
        //     }
        //     if num == 0 {
        //         break;
        //     }
        // }
        // ans

        // iter
        [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ]
        .into_iter()
        .fold((String::new(), num), |(mut s, mut num), (base, unit)| {
            (s + &unit.repeat((num / base) as usize), num % base)
        })
        .0
    }
}