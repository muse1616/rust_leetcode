impl Solution {
    fn dfs(
        candidates: &Vec<i32>,
        index: usize,
        combination: &mut Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
        target: i32,
    ) {
        if target == 0 {
            combinations.push(combination.to_vec());
            return;
        }
        if index >= candidates.len() {
            return;
        }
        for i in index..candidates.len() {
            if i > index && candidates[i] == candidates[i-1] {
                continue;
            }
            if target - candidates[i] >= 0 {
                combination.push(candidates[i]);
                Self::dfs(
                    candidates,
                    i + 1,
                    combination,
                    combinations,
                    target - candidates[i],
                );
                combination.pop();
            } else {
                break;
            }
        }
    }
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut combination = vec![];
        let mut combinations = vec![];

        Self::dfs(&candidates, 0, &mut combination, &mut combinations, target);
        combinations
    }
}