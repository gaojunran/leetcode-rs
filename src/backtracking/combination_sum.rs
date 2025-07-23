struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtrack(0, target, &candidates, &mut path, &mut res);
        res
    }

    fn backtrack(
        start: usize,
        target: i32,
        candidates: &Vec<i32>,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            res.push(path.clone()); // 找到一个可行解
            return;
        }

        for i in start..candidates.len() {
            let num = candidates[i];
            if num > target {
                continue; // 剪枝
            }
            path.push(num); // 做选择
            Self::backtrack(i, target - num, candidates, path, res); // 递归（注意：传 i 而不是 i + 1，表示可以重复选）
            path.pop(); // 撤销选择
        }
    }
}
