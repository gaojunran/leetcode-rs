struct Solution;
impl Solution {
    // 如果一个位置能够到达，那么这个位置左侧所有位置都能到达。
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max = 0;
        for (i, &n) in nums.iter().enumerate() {
            if i > max {
                return false;
            }
            max = max.max(i + n as usize);
        }
        true
    }
}
