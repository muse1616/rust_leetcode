impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len() as i32;
        let mut i = 0;
        while i < n {
            let mut sum_of_gas = 0;
            let mut sum_of_cost = 0;
            let mut cnt = 0;
            while cnt < n {
                let j = (i + cnt) % n;
                sum_of_cost += cost[j as usize];
                sum_of_gas += gas[j as usize];
                if sum_of_cost > sum_of_gas {
                    break;
                }
                cnt += 1;
            }
            if cnt == n {
                return i;
            }
            i = i + cnt + 1;
        }
        -1
    }
}