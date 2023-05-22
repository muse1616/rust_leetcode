impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens.iter().map(|t| t.as_str()) {
            match token {
                op if op == "+" || op == "-" || op == "*" || op == "/" => {
                    if stack.len() < 2 {
                        return -1;
                    }
                    let n2 = stack.pop().unwrap();
                    let n1 = stack.pop().unwrap();
                    if op == "+" {
                        stack.push(n1 + n2);
                    } else if op == "*" {
                        stack.push(n1 * n2);
                    } else if op == "-" {
                        stack.push(n1 - n2);
                    } else {
                        stack.push(n1 / n2);
                    }
                }
                digit => {
                    let digit: i32 = digit.parse().unwrap();
                    stack.push(digit);
                }
            }
        }

        stack[0]
    }
}
