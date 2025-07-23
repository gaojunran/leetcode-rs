struct Solution;

impl Solution {
    // 153. 寻找旋转排序数组中的最小值（返回的是下标）
    fn find_min(nums: &[i32]) -> usize {
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
        right
    }

    // 有序数组中找 target 的下标
    fn lower_bound(nums: &[i32], mut left: usize, mut right: usize, target: i32) -> i32 {
        while left < right {
            // 区间不为空
            let mid = left + (right - left) / 2;
            if nums[mid] >= target {
                right = mid; // 范围缩小到 [left, mid)
            } else {
                left = mid + 1; // 范围缩小到 [mid+1, right)
            }
        }
        if nums[right] == target {
            right as _
        } else {
            -1
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let i = Self::find_min(&nums);
        if target > nums[nums.len() - 1] {
            // target 在第一段
            Self::lower_bound(&nums, 0, i, target) // 左闭右开区间 [0, i)
        } else {
            // target 在第二段
            Self::lower_bound(&nums, i, nums.len(), target) // 左闭右开区间 [i, n)
        }
    }
}
