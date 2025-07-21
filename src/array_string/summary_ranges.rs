struct Solution;
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut i = 0;
        let mut result = vec![];
        for (j, &num) in nums.iter().enumerate() {
            if j + 1 == nums.len() || nums[j + 1] != num + 1 {
                if i == j {
                    // single number
                    result.push(num.to_string());
                } else {
                    // range
                    result.push(format!("{}->{}", nums[i], num));
                }
                i = j + 1;
            }
        }
        result
    }
}
