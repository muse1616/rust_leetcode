impl Solution {
    fn backtrack(
        combination: &mut Vec<char>,
        combinations: &mut Vec<String>,
        index: usize,
        digits: &Vec<char>,
        dic: &std::collections::HashMap<char, Vec<char>>,
    ) {
        if index == digits.len() {
            combinations.push(combination.iter().collect());
            return;
        }
        let c = digits[index];
        for letter in dic[&c].clone() {
            combination.push(letter);
            Self::backtrack(combination, combinations, index + 1, digits, dic);
            combination.pop();
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty(){
            return vec![];
        }
        let digits = digits.chars().collect::<Vec<_>>();
        let dic = std::collections::HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);
        let mut combinations = vec![];
        let mut combination = vec![];
        Self::backtrack(&mut combination, &mut combinations, 0, &digits, &dic);
        combinations
    }
}