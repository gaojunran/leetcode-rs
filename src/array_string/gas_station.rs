struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut total = 0;
        let mut rem = 0;
        let mut start = 0;

        for i in 0..n {
            let diff = gas[i] - cost[i];
            total += diff;
            rem += diff;

            if rem < 0 {
                start = i + 1;
                rem = 0;
            }
        }

        if total < 0 { -1 } else { start as i32 }
    }
}

/// test: gas 2 3 4
/// cost 3 4 3
mod test {
    use super::*;

    #[test]
    fn test() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        let ans = Solution::can_complete_circuit(gas, cost);
        assert_eq!(ans, 1);
    }
}
