struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len = i32::MAX;
        let mut sum = 0;
        let mut left = 0;
        for (right, &num) in nums.iter().enumerate() {
            sum += num;
            while sum >= target {
                min_len = min_len.min((right as i32) - left + 1);
                sum -= nums[left as usize];
                left += 1;
            }
        }
        if min_len == i32::MAX { 0 } else { min_len }
    }
}
