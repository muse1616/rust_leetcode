impl Solution {
    fn backtrack(
        mut left: i32,
        mut right: i32,
        parenthes: &mut Vec<char>,
        n: i32,
        parentheses: &mut Vec<String>,
    ) {
        if left == n && right == n {
            parentheses.push(parenthes.iter().collect());
            return;
        }
        if left < n {
            parenthes.push('(');
            Self::backtrack(left + 1, right, parenthes, n, parentheses);
            parenthes.pop();
        }
        if right < left {
            parenthes.push(')');
            Self::backtrack(left, right + 1, parenthes, n, parentheses);
            parenthes.pop();
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut parenthes = vec![];
        let mut parentheses = vec![];
        Self::backtrack(0, 0, &mut parenthes, n, &mut parentheses);
        parentheses
    }
}