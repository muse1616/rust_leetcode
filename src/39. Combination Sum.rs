

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
        Self::dfs(candidates, index + 1, combination, combinations, target);
        if target - candidates[index] >= 0 {
            combination.push(candidates[index]);
            Self::dfs(
                candidates,
                index,
                combination,
                combinations,
                target - candidates[index],
            );
            combination.pop();
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combination = vec![];
        let mut combinations = vec![];

        Self::dfs(&candidates, 0, &mut combination, &mut combinations, target);
        combinations
    }
}