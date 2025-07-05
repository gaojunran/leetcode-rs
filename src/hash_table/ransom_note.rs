/*
 * @lc app=leetcode.cn id=383 lang=rust
 *
 * [383] 赎金信
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = std::collections::HashMap::new();
        if ransom_note.len() > magazine.len() {
            return false;
        }
        for s in magazine.chars() {
            // if let Some(v) = map.get_mut(&s) {
            //     *v += 1;
            // } else {
            //     map.insert(s, 1);
            // }
            *map.entry(s).or_insert(0) += 1;
        }
        for s in ransom_note.chars() {
            let count = map.entry(s).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1;
            // if let Some(v) = map.get_mut(&s) {
            //     if *v <= 0 {
            //         return false; // if-let chain not stable in leetcode-version rust
            //     }
            //     *v -= 1;
            // } else {
            //     return false;
            // }
        }
        true
    }
}
// @lc code=end
