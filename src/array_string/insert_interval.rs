struct Solution;
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = vec![];
        let mut current = intervals[0].clone();
        for interval in intervals.iter().skip(1) {
            if interval[0] <= current[1] {
                current[1] = current[1].max(interval[1]);
            } else {
                result.push(current);
                current = interval.clone();
            }
        }
        result.push(current);
        result
    }
}
