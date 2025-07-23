struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1; // 左闭右开区间 [0, n-1)
        while left < right {
            // 区间不为空
            let mid = left + (right - left) / 2;
            if nums[mid] < nums[nums.len() - 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[left]
    }
}
