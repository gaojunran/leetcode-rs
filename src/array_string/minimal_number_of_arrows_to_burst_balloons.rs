struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut end = points[0][1];
        let mut count = 1;
        for point in points.iter().skip(1) {
            if point[0] > end {
                count += 1;
                end = point[1];
            }
        }
        count
    }
}
