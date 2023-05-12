impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        // normal
        // let mut carry = 1;
        // let mut ans = vec![];
        // for &n in digits.iter().rev(){
        //     let sum = n + carry;
        //     ans.push(sum%10);
        //     carry = sum/10;
        // }
        // if carry == 1{
        //     ans.push(carry);
        // }
        // ans.reverse();
        // ans

        // functional
        let mut carry = 1;
        let mut digits = digits
            .iter()
            .rev()
            .map(|&n| {
                let ret = n + carry;
                carry = ret / 10;
                ret % 10
            })
            .collect::<Vec<_>>();

        if carry == 1 {
            digits.push(carry);
        }
        digits.reverse();
        digits
    }
}