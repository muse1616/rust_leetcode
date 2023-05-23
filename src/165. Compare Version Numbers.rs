impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1 = version1.split(".").collect::<Vec<_>>();
        let version2 = version2.split(".").collect::<Vec<_>>();

        let (mut i, mut j) = (0, 0);

        while i < version1.len() || j < version2.len() {
            let mut v1 = 0;
            let mut v2 = 0;
            if i < version1.len() {
                v1 = version1[i].parse::<i32>().unwrap_or(0);
                i += 1;
            }
            if j < version2.len() {
                v2 = version2[j].parse::<i32>().unwrap_or(0);
                j += 1;
            }
            if v1 < v2 {
                return -1;
            } else if v1 > v2 {
                return 1;
            }
        }

        0
    }
}
