struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn lower_bound(nums: &Vec<i32>, target: i32) -> usize {
            let mut left = 0;
            let mut right = nums.len(); // 左闭右开

            while left < right {
                let mid = left + (right - left) / 2;
                if nums[mid] < target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        let left = lower_bound(&nums, target);
        let right = lower_bound(&nums, target + 1); // 找 > target 的第一个位置

        if left == nums.len() || nums[left] != target {
            return vec![-1, -1];
        }

        vec![left as i32, (right - 1) as i32]
    }
}
