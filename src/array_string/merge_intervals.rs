struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0])); // 按照左端点从小到大排序
        let mut ans: Vec<Vec<i32>> = vec![];
        for p in intervals {
            if let Some(last) = ans.last_mut() {
                if p[0] <= last[1] {
                    // 可以合并
                    last[1] = last[1].max(p[1]); // 更新右端点最大值
                } else {
                    // 不相交，无法合并
                    ans.push(p); // 新的合并区间
                }
            } else {
                ans.push(p); // 第一个区间
            }
        }
        ans
    }
}
