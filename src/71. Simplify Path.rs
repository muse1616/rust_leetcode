impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path = path.split("/").collect::<Vec<_>>();
        let mut stack = vec![];
        path.iter().for_each(|&s| {
            if s == "" || s == "." {
            } else if s == ".." {
                stack.pop();
            } else {
                stack.push(s);
            }
        });
        format!("/{}", stack.join("/"))
    }
}
