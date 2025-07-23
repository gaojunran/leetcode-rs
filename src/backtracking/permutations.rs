struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res = vec![];
        let mut path = vec![];
        let mut used = vec![false; n];

        fn backtrack(
            nums: &Vec<i32>,
            used: &mut Vec<bool>,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if path.len() == nums.len() {
                res.push(path.clone());
                return;
            }

            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                // 做选择
                used[i] = true;
                path.push(nums[i]);

                backtrack(nums, used, path, res);

                // 撤销选择
                path.pop();
                used[i] = false;
            }
        }

        backtrack(&nums, &mut used, &mut path, &mut res);
        res
    }
}
