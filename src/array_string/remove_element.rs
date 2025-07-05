struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while i < nums.len() {
            if nums[i] != val {
                nums[j] = nums[i];
                j += 1;
            }
            i += 1;
        }
        j as i32
    }
}
