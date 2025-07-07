struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..prices.len() {
            ans += std::cmp::max(0, prices[i] - prices[i - 1])
        }
        ans
    }
}
