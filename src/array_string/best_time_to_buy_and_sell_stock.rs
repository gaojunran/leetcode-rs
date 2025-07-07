use std::cmp::max;
struct Solution;
impl Solution {
    // 假设我们在第 i 天卖出股票得到了最大的利润，那我们的买入价格一定是 前i-1天 中最低价格时买入的。
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            if price < min_price {
                min_price = price;
            } else if price - min_price > max_profit {
                max_profit = price - min_price;
            }
        }
        max_profit
    }
}
