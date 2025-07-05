/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        // key: nums[i], value: i
        for (idx, num) in nums.iter().enumerate() {
            if let Some(i) = map.get(&(target - num)) {
                return vec![*i, idx as i32];
            } else {
                map.insert(num, idx as i32);
            }
        }
        vec![]
    }
}
// @lc code=end
