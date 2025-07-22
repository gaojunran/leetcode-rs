struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut l = 0;
        let mut r = n;
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32
    }
}
