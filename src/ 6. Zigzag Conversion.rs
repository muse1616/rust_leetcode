impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // normal
        // if num_rows == 1{
        //     return s;
        // }
        // let mut v = vec![vec![];num_rows as usize];

        // let s = s.chars();
        // let mut row:i32 = 0;
        // let mut dir:i32 = -1;
        // for c in s{
        //     v[row as usize].push(c);
        //     if row == num_rows - 1 || row == 0{
        //         dir = -dir;
        //     }
        //     row+=dir;
        // }
        // let v = v.concat();
        // v.iter().collect()

        // iter
        let num_rows = num_rows as usize;
        let mut rows = vec![String::new();num_rows];
        let iter = (0..num_rows).chain((1..num_rows-1).rev()).cycle();
        iter.zip(s.chars()).for_each(|(i,v)|{rows[i].push(v)});
        // rows.concat()
        rows.into_iter().collect()

    }
}