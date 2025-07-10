struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1; n];

        // 1. 前缀乘积
        for i in 1..n {
            answer[i] = answer[i - 1] * nums[i - 1];
        }

        // 2. 后缀乘积 + 直接乘到 answer 中
        let mut right = 1;
        for i in (0..n).rev() {
            answer[i] *= right;
            right *= nums[i];
        }

        answer
    }
}
