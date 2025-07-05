use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = HashSet::<i32>::from_iter(nums);
        let mut longest = 0;
        for &n in &set {
            if set.contains(&(n - 1)) {
                continue;
            }
            let mut curr = n;
            let mut length = 1;
            while set.contains(&(curr + 1)) {
                curr += 1;
                length += 1;
            }
            longest = longest.max(length);
        }
        longest
    }
}
