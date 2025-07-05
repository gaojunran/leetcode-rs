struct Solution;
impl Solution {
    // pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    //     // let backup = nums.clone();
    //     for _ in 0..k {
    //         nums.insert(0, nums[nums.len() - 1]);
    //         nums.pop();
    //     }
    // }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let backup = nums.clone();
        for (i, &num) in backup.iter().enumerate() {
            let idx = (i + k as usize) % backup.len();
            nums[idx] = num;
        }
    }
}
