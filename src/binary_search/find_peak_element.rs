struct Solution;
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1; // 改成闭区间 [0, n-1]

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] > nums[mid + 1] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left as i32
}
