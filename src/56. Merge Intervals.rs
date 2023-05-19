impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // let mut ans = vec![];
        // intervals.sort_by(|x, y| x[0].partial_cmp(&y[0]).unwrap());
        // for interval in intervals.into_iter() {
        //     if ans.len() == 0 {
        //         ans.push(interval);
        //     } else {
        //         if interval[0] <= ans.last().unwrap()[1] {
        //             if interval[1] > ans.last().unwrap()[1] {
        //                 ans.last_mut().unwrap()[1] = interval[1];
        //             }
        //         } else {
        //             ans.push(interval);
        //         }
        //     }
        // }
        // ans
        let mut ans = vec![];
        intervals.sort_by(|x, y| x[0].partial_cmp(&y[0]).unwrap());
        let mut interval = vec![intervals[0][0], intervals[0][1]];
        intervals.into_iter().skip(1).for_each(|x| {
            if x[0] > interval[1] {
                ans.push(interval.clone());
                interval[0] = x[0];
            }
            interval[1] = interval[1].max(x[1]);
        });
        ans.push(interval);
        ans
    }
}