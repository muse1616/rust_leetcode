impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = numerator as i64;
        let denominator = denominator as i64;

        if numerator % denominator == 0 {
            return (numerator / denominator).to_string();
        }
        let mut negative = false;
        if (numerator > 0 && denominator < 0) || (numerator < 0 && denominator > 0) {
            negative = true;
        }
        let numerator = numerator.abs();
        let denominator = denominator.abs();

        let mut integer_part = String::new();
        if negative {
            integer_part.push_str("-");
        }
        integer_part.push_str((numerator / denominator).to_string().as_str());

        let mut fraction: Vec<char> = vec![];
        let mut map = std::collections::HashMap::new();
        let mut remainder = (numerator % denominator).abs() as i64;
        let mut index = 0;
        while remainder != 0 && !map.contains_key(&remainder) {
            map.insert(remainder, index);
            remainder *= 10;
            fraction.push(std::char::from_digit((remainder / denominator) as u32, 10).unwrap());
            remainder = remainder % denominator;
            index += 1;
        }
        if remainder != 0 {
            fraction.insert(map[&remainder], '(');
            fraction.push(')');
        }
        format!("{}.{}", integer_part, fraction.iter().collect::<String>())
    }
}