impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut placed = false;
        let (mut left, mut right) = (new_interval[0], new_interval[1]);
        intervals.into_iter().for_each(|interval| {
            if interval[0] > right {
                if !placed {
                    ans.push(vec![left, right]);
                    placed = true;
                }
                ans.push(interval);
            } else if interval[1] < left {
                ans.push(interval);
            } else {
                left = left.min(interval[0]);
                right = right.max(interval[1]);
            }
        });
        if !placed {
            ans.push(vec![left, right]);
        }
        ans
    }
}
