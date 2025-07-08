struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common = String::new();
        for (i, &c) in strs[0].as_bytes().iter().enumerate() {
            if strs.iter().any(|s| s.len() < i + 1) {
                break;
            }
            if strs.iter().all(|s| s.as_bytes()[i] == c) {
                common.push(c as char);
            } else {
                break;
            }
        }
        common
    }
}
