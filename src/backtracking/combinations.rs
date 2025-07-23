struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut path = vec![];

        fn dfs(start: i32, n: i32, k: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if path.len() == k as usize {
                ans.push(path.clone());
                return;
            }

            // 剪枝优化：i 最多只能到 n - (k - path.len()) + 1
            for i in start..=n - (k - path.len() as i32) + 1 {
                path.push(i);
                dfs(i + 1, n, k, path, ans);
                path.pop(); // 回溯
            }
        }

        dfs(1, n, k, &mut path, &mut ans);
        ans
    }
}
