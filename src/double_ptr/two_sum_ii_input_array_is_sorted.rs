struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // let mut i = 0;
        let mut j = numbers.len() - 1;
        for i in 0..numbers.len() {
            while numbers[i] + numbers[j] >= target {
                if numbers[i] + numbers[j] == target {
                    return vec![i as i32 + 1, j as i32 + 1];
                }
                j -= 1;
            }
        }
        vec![]
    }
}

// impl Solution {
//     pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
//         let (mut l, mut r) = (0, numbers.len() - 1);
//         while numbers[l] + numbers[r] != target {
//             if numbers[l] + numbers[r] < target {
//                 l += 1;
//             } else {
//                 r -= 1;
//             }
//         }
//         vec![l as i32 + 1, r as i32 + 1]
//     }
// }
